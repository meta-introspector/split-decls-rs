mkuse!{use crate :: cell :: OnceCell ;}
mkuse!{use crate :: hint :: spin_loop ;}
mkuse!{use crate :: mem ;}
mkuse!{use crate :: ptr :: { self , NonNull , null_mut , without_provenance_mut } ;}
mkuse!{use crate :: sync :: atomic :: Ordering :: { AcqRel , Acquire , Relaxed , Release } ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicBool , AtomicPtr } ;}
mkuse!{use crate :: thread :: { self , Thread } ;}
mkitem!{# [doc = " The atomic lock state."] type AtomicState = Atomic < State > ;}
mkitem!{# [doc = " The inner lock state."] type State = * mut () ;}
mkitem!{const UNLOCKED : State = without_provenance_mut (0) ;}
mkitem!{const LOCKED : usize = 1 << 0 ;}
mkitem!{const QUEUED : usize = 1 << 1 ;}
mkitem!{const QUEUE_LOCKED : usize = 1 << 2 ;}
mkitem!{const DOWNGRADED : usize = 1 << 3 ;}
mkitem!{const SINGLE : usize = 1 << 4 ;}
mkitem!{const STATE : usize = DOWNGRADED | QUEUE_LOCKED | QUEUED | LOCKED ;}
mkitem!{const NODE_MASK : usize = ! STATE ;}
mkitem!{# [doc = " Locking uses exponential backoff. `SPIN_COUNT` indicates how many times the locking operation"] # [doc = " will be retried."] # [doc = ""] # [doc = " In other words, `spin_loop` will be called `2.pow(SPIN_COUNT) - 1` times."] const SPIN_COUNT : usize = 7 ;}

macro_rules! write_lock_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_lock in module {}", module_path!());
    };
}

mkfn!{
    write_lock_introspect!();
    # [doc = " Marks the state as write-locked, if possible."] # [inline] fn write_lock (state : State) -> Option < State > { if state . addr () & LOCKED == 0 { Some (state . map_addr (| addr | addr | LOCKED)) } else { None } }
}

macro_rules! read_lock_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function read_lock in module {}", module_path!());
    };
}

mkfn!{
    read_lock_introspect!();
    # [doc = " Marks the state as read-locked, if possible."] # [inline] fn read_lock (state : State) -> Option < State > { if state . addr () & QUEUED == 0 && state . addr () != LOCKED { Some (without_provenance_mut (state . addr () . checked_add (SINGLE) ? | LOCKED)) } else { None } }
}

macro_rules! to_node_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_node in module {}", module_path!());
    };
}

mkfn!{
    to_node_introspect!();
    # [doc = " Converts a `State` into a `Node` by masking out the bottom bits of the state, assuming that the"] # [doc = " state points to a queue node."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The state must contain a valid pointer to a queue node."] # [inline] unsafe fn to_node (state : State) -> NonNull < Node > { unsafe { NonNull :: new_unchecked (state . mask (NODE_MASK)) . cast () } }
}
mkitem!{mkstruct!{# [doc = " The representation of a thread waiting on the lock queue."] # [doc = ""] # [doc = " We initialize these `Node`s on thread execution stacks to avoid allocation."] # [doc = ""] # [doc = " Note that we need an alignment of 16 to ensure that the last 4 bits of any"] # [doc = " pointers to `Node`s are always zeroed (for the bit flags described in the"] # [doc = " module-level documentation)."] # [repr (align (16))] struct Node { next : AtomicLink , prev : AtomicLink , tail : AtomicLink , write : bool , thread : OnceCell < Thread > , completed : Atomic < bool > , }}}
mkitem!{mkstruct!{# [doc = " An atomic node pointer with relaxed operations."] struct AtomicLink (Atomic < * mut Node >) ;}}
mkitem!{mkimpl!{impl AtomicLink { fn new (v : Option < NonNull < Node > >) -> AtomicLink { AtomicLink (AtomicPtr :: new (v . map_or (null_mut () , NonNull :: as_ptr))) } fn get (& self) -> Option < NonNull < Node > > { NonNull :: new (self . 0 . load (Relaxed)) } fn set (& self , v : Option < NonNull < Node > >) { self . 0 . store (v . map_or (null_mut () , NonNull :: as_ptr) , Relaxed) ; } }}}
mkitem!{mkimpl!{impl Node { # [doc = " Creates a new queue node."] fn new (write : bool) -> Node { Node { next : AtomicLink :: new (None) , prev : AtomicLink :: new (None) , tail : AtomicLink :: new (None) , write , thread : OnceCell :: new () , completed : AtomicBool :: new (false) , } } # [doc = " Prepare this node for waiting."] fn prepare (& mut self) { self . thread . get_or_init (thread :: current_or_unnamed) ; self . completed = AtomicBool :: new (false) ; } # [doc = " Wait until this node is marked as [`complete`](Node::complete)d by another thread."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " May only be called from the thread that created the node."] unsafe fn wait (& self) { while ! self . completed . load (Acquire) { unsafe { self . thread . get () . unwrap () . park () ; } } } # [doc = " Atomically mark this node as completed."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `node` must point to a valid `Node`, and the node may not outlive this call."] unsafe fn complete (node : NonNull < Node >) { let thread = unsafe { node . as_ref () . thread . get () . unwrap () . clone () } ; unsafe { node . as_ref () . completed . store (true , Release) ; } thread . unpark () ; } }}}

macro_rules! find_tail_and_add_backlinks_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_tail_and_add_backlinks in module {}", module_path!());
    };
}

mkfn!{
    find_tail_and_add_backlinks_introspect!();
    # [doc = " Traverse the queue and find the tail, adding backlinks to the queue while traversing."] # [doc = ""] # [doc = " This may be called from multiple threads at the same time as long as the queue is not being"] # [doc = " modified (this happens when unlocking multiple readers)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " * `head` must point to a node in a valid queue."] # [doc = " * `head` must be in front of the previous head node that was used to perform the last removal."] # [doc = " * The part of the queue starting with `head` must not be modified during this call."] unsafe fn find_tail_and_add_backlinks (head : NonNull < Node >) -> NonNull < Node > { let mut current = head ; let tail = loop { let c = unsafe { current . as_ref () } ; if let Some (tail) = c . tail . get () { break tail ; } unsafe { let next = c . next . get () . unwrap_unchecked () ; next . as_ref () . prev . set (Some (current)) ; current = next ; } } ; unsafe { head . as_ref () . tail . set (Some (tail)) ; tail } }
}

macro_rules! complete_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function complete_all in module {}", module_path!());
    };
}

mkfn!{
    complete_all_introspect!();
    # [doc = " [`complete`](Node::complete)s all threads in the queue ending with `tail`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " * `tail` must be a valid tail of a fully linked queue."] # [doc = " * The current thread must have exclusive access to that queue."] unsafe fn complete_all (tail : NonNull < Node >) { let mut current = tail ; loop { let prev = unsafe { current . as_ref () . prev . get () } ; unsafe { Node :: complete (current) ; } match prev { Some (prev) => current = prev , None => return , } } }
}
mkitem!{mkstruct!{# [doc = " A type to guard against the unwinds of stacks that nodes are located on due to panics."] struct PanicGuard ;}}
mkitem!{mkimpl!{impl Drop for PanicGuard { fn drop (& mut self) { rtabort ! ("tried to drop node in intrusive list.") ; } }}}
mkitem!{mkstruct!{# [doc = " The public inner `RwLock` type."] pub struct RwLock { state : AtomicState , }}}
mkitem!{mkimpl!{impl RwLock { # [inline] pub const fn new () -> RwLock { RwLock { state : AtomicPtr :: new (UNLOCKED) } } # [inline] pub fn try_read (& self) -> bool { self . state . fetch_update (Acquire , Relaxed , read_lock) . is_ok () } # [inline] pub fn read (& self) { if ! self . try_read () { self . lock_contended (false) } } # [inline] pub fn try_write (& self) -> bool { self . state . fetch_or (LOCKED , Acquire) . addr () & LOCKED == 0 } # [inline] pub fn write (& self) { if ! self . try_write () { self . lock_contended (true) } } # [cold] fn lock_contended (& self , write : bool) { let mut node = Node :: new (write) ; let mut state = self . state . load (Relaxed) ; let mut count = 0 ; let update_fn = if write { write_lock } else { read_lock } ; loop { if let Some (next) = update_fn (state) { match self . state . compare_exchange_weak (state , next , Acquire , Relaxed) { Ok (_) => return , Err (new) => state = new , } continue ; } else if state . addr () & QUEUED == 0 && count < SPIN_COUNT { for _ in 0 .. (1 << count) { spin_loop () ; } state = self . state . load (Relaxed) ; count += 1 ; continue ; } node . prepare () ; node . next . 0 = AtomicPtr :: new (state . mask (NODE_MASK) . cast ()) ; node . prev = AtomicLink :: new (None) ; let mut next = ptr :: from_ref (& node) . map_addr (| addr | addr | QUEUED | (state . addr () & (DOWNGRADED | LOCKED))) as State ; let mut is_queue_locked = false ; if state . addr () & QUEUED == 0 { node . tail . set (Some (NonNull :: from (& node))) ; } else { node . tail . set (None) ; next = next . map_addr (| addr | addr | QUEUE_LOCKED) ; is_queue_locked = state . addr () & QUEUE_LOCKED == 0 ; } if let Err (new) = self . state . compare_exchange_weak (state , next , AcqRel , Relaxed) { state = new ; continue ; } let guard = PanicGuard ; if is_queue_locked { unsafe { self . unlock_queue (next) ; } } unsafe { node . wait () ; } mem :: forget (guard) ; state = self . state . load (Relaxed) ; count = 0 ; } } # [inline] pub unsafe fn read_unlock (& self) { match self . state . fetch_update (Release , Acquire , | state | { if state . addr () & QUEUED == 0 { let count = state . addr () - (SINGLE | LOCKED) ; Some (if count > 0 { without_provenance_mut (count | LOCKED) } else { UNLOCKED }) } else if state . addr () & DOWNGRADED != 0 { Some (state . mask (! (DOWNGRADED | LOCKED))) } else { None } }) { Ok (_) => { } Err (state) => unsafe { self . read_unlock_contended (state) } , } } # [doc = " # Safety"] # [doc = ""] # [doc = " * There must be threads queued on the lock."] # [doc = " * `state` must be a pointer to a node in a valid queue."] # [doc = " * There cannot be a `downgrade` in progress."] # [cold] unsafe fn read_unlock_contended (& self , state : State) { let tail = unsafe { find_tail_and_add_backlinks (to_node (state)) . as_ref () } ; let was_last = tail . next . 0 . fetch_byte_sub (SINGLE , AcqRel) . addr () - SINGLE == 0 ; if was_last { unsafe { self . unlock_contended (state) } } } # [inline] pub unsafe fn write_unlock (& self) { if let Err (state) = self . state . compare_exchange (without_provenance_mut (LOCKED) , UNLOCKED , Release , Relaxed) { unsafe { self . unlock_contended (state) } } } # [doc = " # Safety"] # [doc = ""] # [doc = " * The lock must be exclusively owned by this thread."] # [doc = " * There must be threads queued on the lock."] # [doc = " * `state` must be a pointer to a node in a valid queue."] # [doc = " * There cannot be a `downgrade` in progress."] # [cold] unsafe fn unlock_contended (& self , state : State) { debug_assert_eq ! (state . addr () & (DOWNGRADED | QUEUED | LOCKED) , QUEUED | LOCKED) ; let mut current = state ; loop { if current . addr () & QUEUE_LOCKED != 0 { let next = current . mask (! LOCKED) ; match self . state . compare_exchange_weak (current , next , Release , Relaxed) { Ok (_) => return , Err (new) => { current = new ; continue ; } } } let next = current . map_addr (| addr | (addr & ! LOCKED) | QUEUE_LOCKED) ; match self . state . compare_exchange_weak (current , next , AcqRel , Relaxed) { Ok (_) => { unsafe { self . unlock_queue (next) } ; return ; } Err (new) => current = new , } } } # [doc = " # Safety"] # [doc = ""] # [doc = " * The lock must be write-locked by this thread."] # [inline] pub unsafe fn downgrade (& self) { if let Err (state) = self . state . compare_exchange (without_provenance_mut (LOCKED) , without_provenance_mut (SINGLE | LOCKED) , Release , Relaxed ,) { unsafe { self . downgrade_slow (state) } } } # [doc = " Downgrades the lock from write-locked to read-locked in the case that there are threads"] # [doc = " waiting on the wait queue."] # [doc = ""] # [doc = " This function will either wake up all of the waiters on the wait queue or designate the"] # [doc = " current holder of the queue lock to wake up all of the waiters instead. Once the waiters"] # [doc = " wake up, they will continue in the execution loop of `lock_contended`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " * The lock must be write-locked by this thread."] # [doc = " * `state` must be a pointer to a node in a valid queue."] # [doc = " * There must be threads queued on the lock."] # [cold] unsafe fn downgrade_slow (& self , mut state : State) { debug_assert_eq ! (state . addr () & (DOWNGRADED | QUEUED | LOCKED) , QUEUED | LOCKED) ; loop { if state . addr () & QUEUE_LOCKED != 0 { let next = state . map_addr (| addr | addr | DOWNGRADED) ; match self . state . compare_exchange_weak (state , next , Release , Relaxed) { Ok (_) => return , Err (new) => state = new , } } else { let next = ptr :: without_provenance_mut (SINGLE | LOCKED) ; if let Err (new) = self . state . compare_exchange_weak (state , next , AcqRel , Relaxed) { state = new ; continue ; } let tail = unsafe { find_tail_and_add_backlinks (to_node (state)) } ; unsafe { complete_all (tail) } ; return ; } } } # [doc = " Unlocks the queue. Wakes up all threads if a downgrade was requested, otherwise wakes up the"] # [doc = " next eligible thread(s) if the lock is unlocked."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " * The queue lock must be held by the current thread."] # [doc = " * `state` must be a pointer to a node in a valid queue."] # [doc = " * There must be threads queued on the lock."] unsafe fn unlock_queue (& self , mut state : State) { debug_assert_eq ! (state . addr () & (QUEUED | QUEUE_LOCKED) , QUEUED | QUEUE_LOCKED) ; loop { let tail = unsafe { find_tail_and_add_backlinks (to_node (state)) } ; if state . addr () & (DOWNGRADED | LOCKED) == LOCKED { match self . state . compare_exchange_weak (state , state . mask (! QUEUE_LOCKED) , Release , Acquire ,) { Ok (_) => return , Err (new) => { state = new ; continue ; } } } let downgrade = state . addr () & DOWNGRADED != 0 ; let is_writer = unsafe { tail . as_ref () . write } ; if ! downgrade && is_writer && let Some (prev) = unsafe { tail . as_ref () . prev . get () } { unsafe { to_node (state) . as_ref () . tail . set (Some (prev)) ; } let next = state . mask (! QUEUE_LOCKED) ; if let Err (new) = self . state . compare_exchange_weak (state , next , Release , Acquire) { unsafe { to_node (state) . as_ref () . tail . set (Some (tail)) ; } state = new ; continue ; } unsafe { return Node :: complete (tail) ; } } else { let next = if downgrade { ptr :: without_provenance_mut (SINGLE | LOCKED) } else { UNLOCKED } ; if let Err (new) = self . state . compare_exchange_weak (state , next , Release , Acquire) { state = new ; continue ; } unsafe { return complete_all (tail) ; } } } } }}}