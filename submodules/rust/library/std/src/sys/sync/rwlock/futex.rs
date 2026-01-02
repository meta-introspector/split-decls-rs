mkuse!{use crate :: sync :: atomic :: Ordering :: { Acquire , Relaxed , Release } ;}
mkuse!{use crate :: sys :: futex :: { Futex , Primitive , futex_wait , futex_wake , futex_wake_all } ;}
mkitem!{mkstruct!{pub struct RwLock { state : Futex , writer_notify : Futex , }}}
mkitem!{const READ_LOCKED : Primitive = 1 ;}
mkitem!{const MASK : Primitive = (1 << 30) - 1 ;}
mkitem!{const WRITE_LOCKED : Primitive = MASK ;}
mkitem!{const DOWNGRADE : Primitive = READ_LOCKED . wrapping_sub (WRITE_LOCKED) ;}
mkitem!{const MAX_READERS : Primitive = MASK - 1 ;}
mkitem!{const READERS_WAITING : Primitive = 1 << 30 ;}
mkitem!{const WRITERS_WAITING : Primitive = 1 << 31 ;}

macro_rules! is_unlocked_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_unlocked in module {}", module_path!());
    };
}

mkfn!{
    is_unlocked_introspect!();
    # [inline] fn is_unlocked (state : Primitive) -> bool { state & MASK == 0 }
}

macro_rules! is_write_locked_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_write_locked in module {}", module_path!());
    };
}

mkfn!{
    is_write_locked_introspect!();
    # [inline] fn is_write_locked (state : Primitive) -> bool { state & MASK == WRITE_LOCKED }
}

macro_rules! has_readers_waiting_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_readers_waiting in module {}", module_path!());
    };
}

mkfn!{
    has_readers_waiting_introspect!();
    # [inline] fn has_readers_waiting (state : Primitive) -> bool { state & READERS_WAITING != 0 }
}

macro_rules! has_writers_waiting_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_writers_waiting in module {}", module_path!());
    };
}

mkfn!{
    has_writers_waiting_introspect!();
    # [inline] fn has_writers_waiting (state : Primitive) -> bool { state & WRITERS_WAITING != 0 }
}

macro_rules! is_read_lockable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_read_lockable in module {}", module_path!());
    };
}

mkfn!{
    is_read_lockable_introspect!();
    # [inline] fn is_read_lockable (state : Primitive) -> bool { state & MASK < MAX_READERS && ! has_readers_waiting (state) && ! has_writers_waiting (state) }
}

macro_rules! is_read_lockable_after_wakeup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_read_lockable_after_wakeup in module {}", module_path!());
    };
}

mkfn!{
    is_read_lockable_after_wakeup_introspect!();
    # [inline] fn is_read_lockable_after_wakeup (state : Primitive) -> bool { state & MASK < MAX_READERS && ! has_readers_waiting (state) && ! is_write_locked (state) && ! is_unlocked (state) }
}

macro_rules! has_reached_max_readers_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_reached_max_readers in module {}", module_path!());
    };
}

mkfn!{
    has_reached_max_readers_introspect!();
    # [inline] fn has_reached_max_readers (state : Primitive) -> bool { state & MASK == MAX_READERS }
}
mkitem!{mkimpl!{impl RwLock { # [inline] pub const fn new () -> Self { Self { state : Futex :: new (0) , writer_notify : Futex :: new (0) } } # [inline] pub fn try_read (& self) -> bool { self . state . fetch_update (Acquire , Relaxed , | s | is_read_lockable (s) . then (| | s + READ_LOCKED)) . is_ok () } # [inline] pub fn read (& self) { let state = self . state . load (Relaxed) ; if ! is_read_lockable (state) || self . state . compare_exchange_weak (state , state + READ_LOCKED , Acquire , Relaxed) . is_err () { self . read_contended () ; } } # [doc = " # Safety"] # [doc = ""] # [doc = " The `RwLock` must be read-locked (N readers) in order to call this."] # [inline] pub unsafe fn read_unlock (& self) { let state = self . state . fetch_sub (READ_LOCKED , Release) - READ_LOCKED ; debug_assert ! (! has_readers_waiting (state) || has_writers_waiting (state)) ; if is_unlocked (state) && has_writers_waiting (state) { self . wake_writer_or_readers (state) ; } } # [cold] fn read_contended (& self) { let mut has_slept = false ; let mut state = self . spin_read () ; loop { if (has_slept && is_read_lockable_after_wakeup (state)) || is_read_lockable (state) { match self . state . compare_exchange_weak (state , state + READ_LOCKED , Acquire , Relaxed) { Ok (_) => return , Err (s) => { state = s ; continue ; } } } assert ! (! has_reached_max_readers (state) , "too many active read locks on RwLock") ; if ! has_readers_waiting (state) { if let Err (s) = self . state . compare_exchange (state , state | READERS_WAITING , Relaxed , Relaxed) { state = s ; continue ; } } futex_wait (& self . state , state | READERS_WAITING , None) ; has_slept = true ; state = self . spin_read () ; } } # [inline] pub fn try_write (& self) -> bool { self . state . fetch_update (Acquire , Relaxed , | s | is_unlocked (s) . then (| | s + WRITE_LOCKED)) . is_ok () } # [inline] pub fn write (& self) { if self . state . compare_exchange_weak (0 , WRITE_LOCKED , Acquire , Relaxed) . is_err () { self . write_contended () ; } } # [doc = " # Safety"] # [doc = ""] # [doc = " The `RwLock` must be write-locked (single writer) in order to call this."] # [inline] pub unsafe fn write_unlock (& self) { let state = self . state . fetch_sub (WRITE_LOCKED , Release) - WRITE_LOCKED ; debug_assert ! (is_unlocked (state)) ; if has_writers_waiting (state) || has_readers_waiting (state) { self . wake_writer_or_readers (state) ; } } # [doc = " # Safety"] # [doc = ""] # [doc = " The `RwLock` must be write-locked (single writer) in order to call this."] # [inline] pub unsafe fn downgrade (& self) { let state = self . state . fetch_add (DOWNGRADE , Release) ; debug_assert ! (is_write_locked (state) , "RwLock must be write locked to call `downgrade`") ; if has_readers_waiting (state) { self . state . fetch_sub (READERS_WAITING , Relaxed) ; futex_wake_all (& self . state) ; } } # [cold] fn write_contended (& self) { let mut state = self . spin_write () ; let mut other_writers_waiting = 0 ; loop { if is_unlocked (state) { match self . state . compare_exchange_weak (state , state | WRITE_LOCKED | other_writers_waiting , Acquire , Relaxed ,) { Ok (_) => return , Err (s) => { state = s ; continue ; } } } if ! has_writers_waiting (state) { if let Err (s) = self . state . compare_exchange (state , state | WRITERS_WAITING , Relaxed , Relaxed) { state = s ; continue ; } } other_writers_waiting = WRITERS_WAITING ; let seq = self . writer_notify . load (Acquire) ; state = self . state . load (Relaxed) ; if is_unlocked (state) || ! has_writers_waiting (state) { continue ; } futex_wait (& self . writer_notify , seq , None) ; state = self . spin_write () ; } } # [doc = " Wakes up waiting threads after unlocking."] # [doc = ""] # [doc = " If both are waiting, this will wake up only one writer, but will fall"] # [doc = " back to waking up readers if there was no writer to wake up."] # [cold] fn wake_writer_or_readers (& self , mut state : Primitive) { assert ! (is_unlocked (state)) ; if state == WRITERS_WAITING { match self . state . compare_exchange (state , 0 , Relaxed , Relaxed) { Ok (_) => { self . wake_writer () ; return ; } Err (s) => { state = s ; } } } if state == READERS_WAITING + WRITERS_WAITING { if self . state . compare_exchange (state , READERS_WAITING , Relaxed , Relaxed) . is_err () { return ; } if self . wake_writer () { return ; } state = READERS_WAITING ; } if state == READERS_WAITING { if self . state . compare_exchange (state , 0 , Relaxed , Relaxed) . is_ok () { futex_wake_all (& self . state) ; } } } # [doc = " This wakes one writer and returns true if we woke up a writer that was"] # [doc = " blocked on futex_wait."] # [doc = ""] # [doc = " If this returns false, it might still be the case that we notified a"] # [doc = " writer that was about to go to sleep."] fn wake_writer (& self) -> bool { self . writer_notify . fetch_add (1 , Release) ; futex_wake (& self . writer_notify) } # [doc = " Spin for a while, but stop directly at the given condition."] # [inline] fn spin_until (& self , f : impl Fn (Primitive) -> bool) -> Primitive { let mut spin = 100 ; loop { let state = self . state . load (Relaxed) ; if f (state) || spin == 0 { return state ; } crate :: hint :: spin_loop () ; spin -= 1 ; } } # [inline] fn spin_write (& self) -> Primitive { self . spin_until (| state | is_unlocked (state) || has_writers_waiting (state)) } # [inline] fn spin_read (& self) -> Primitive { self . spin_until (| state | { ! is_write_locked (state) || has_readers_waiting (state) || has_writers_waiting (state) }) } }}}