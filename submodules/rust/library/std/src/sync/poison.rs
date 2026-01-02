mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: condvar :: Condvar ;}
mkuse!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] pub use self :: mutex :: MappedMutexGuard ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: mutex :: { Mutex , MutexGuard } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [expect (deprecated)] pub use self :: once :: ONCE_INIT ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: once :: { Once , OnceState } ;}
mkuse!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] pub use self :: rwlock :: { MappedRwLockReadGuard , MappedRwLockWriteGuard } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: rwlock :: { RwLock , RwLockReadGuard , RwLockWriteGuard } ;}
mkuse!{use crate :: error :: Error ;}
mkuse!{use crate :: fmt ;}
mkuse!{# [cfg (panic = "unwind")] use crate :: sync :: atomic :: { Atomic , AtomicBool , Ordering } ;}
mkuse!{# [cfg (panic = "unwind")] use crate :: thread ;}
mkmod!{condvar, { 
                getname!(condvar);
                getsrc!(condvar);
                getpath!(condvar);
                get_deps!(condvar);
                get_crates!(condvar);
                mkinclude!(condvar);
                 
            }}
mkmod!{mutex, { 
                getname!(mutex);
                getsrc!(mutex);
                getpath!(mutex);
                get_deps!(mutex);
                get_crates!(mutex);
                mkinclude!(mutex);
                 
            }}
mkmod!{once, { 
                getname!(once);
                getsrc!(once);
                getpath!(once);
                get_deps!(once);
                get_crates!(once);
                mkinclude!(once);
                 
            }}
mkmod!{rwlock, { 
                getname!(rwlock);
                getsrc!(rwlock);
                getpath!(rwlock);
                get_deps!(rwlock);
                get_crates!(rwlock);
                mkinclude!(rwlock);
                 
            }}
mkitem!{mkstruct!{pub (crate) struct Flag { # [cfg (panic = "unwind")] failed : Atomic < bool > , }}}
mkitem!{mkimpl!{impl Flag { # [inline] pub const fn new () -> Flag { Flag { # [cfg (panic = "unwind")] failed : AtomicBool :: new (false) , } } # [doc = " Checks the flag for an unguarded borrow, where we only care about existing poison."] # [inline] pub fn borrow (& self) -> LockResult < () > { if self . get () { Err (PoisonError :: new (())) } else { Ok (()) } } # [doc = " Checks the flag for a guarded borrow, where we may also set poison when `done`."] # [inline] pub fn guard (& self) -> LockResult < Guard > { let ret = Guard { # [cfg (panic = "unwind")] panicking : thread :: panicking () , } ; if self . get () { Err (PoisonError :: new (ret)) } else { Ok (ret) } } # [inline] # [cfg (panic = "unwind")] pub fn done (& self , guard : & Guard) { if ! guard . panicking && thread :: panicking () { self . failed . store (true , Ordering :: Relaxed) ; } } # [inline] # [cfg (not (panic = "unwind"))] pub fn done (& self , _guard : & Guard) { } # [inline] # [cfg (panic = "unwind")] pub fn get (& self) -> bool { self . failed . load (Ordering :: Relaxed) } # [inline (always)] # [cfg (not (panic = "unwind"))] pub fn get (& self) -> bool { false } # [inline] pub fn clear (& self) { # [cfg (panic = "unwind")] self . failed . store (false , Ordering :: Relaxed) } }}}
mkitem!{mkstruct!{# [derive (Clone)] pub (crate) struct Guard { # [cfg (panic = "unwind")] panicking : bool , }}}
mkitem!{mkstruct!{# [doc = " A type of error which can be returned whenever a lock is acquired."] # [doc = ""] # [doc = " Both [`Mutex`]es and [`RwLock`]s are poisoned whenever a thread fails while the lock"] # [doc = " is held. The precise semantics for when a lock is poisoned is documented on"] # [doc = " each lock. For a lock in the poisoned state, unless the state is cleared manually,"] # [doc = " all future acquisitions will return this error."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::{Arc, Mutex};"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let mutex = Arc::new(Mutex::new(1));"] # [doc = ""] # [doc = " // poison the mutex"] # [doc = " let c_mutex = Arc::clone(&mutex);"] # [doc = " let _ = thread::spawn(move || {"] # [doc = "     let mut data = c_mutex.lock().unwrap();"] # [doc = "     *data = 2;"] # [doc = "     panic!();"] # [doc = " }).join();"] # [doc = ""] # [doc = " match mutex.lock() {"] # [doc = "     Ok(_) => unreachable!(),"] # [doc = "     Err(p_err) => {"] # [doc = "         let data = p_err.get_ref();"] # [doc = "         println!(\"recovered: {data}\");"] # [doc = "     }"] # [doc = " };"] # [doc = " ```"] # [doc = " [`Mutex`]: crate::sync::Mutex"] # [doc = " [`RwLock`]: crate::sync::RwLock"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct PoisonError < T > { data : T , # [cfg (not (panic = "unwind"))] _never : ! , }}}
mkitem!{mkenum!{# [doc = " An enumeration of possible errors associated with a [`TryLockResult`] which"] # [doc = " can occur while trying to acquire a lock, from the [`try_lock`] method on a"] # [doc = " [`Mutex`] or the [`try_read`] and [`try_write`] methods on an [`RwLock`]."] # [doc = ""] # [doc = " [`try_lock`]: crate::sync::Mutex::try_lock"] # [doc = " [`try_read`]: crate::sync::RwLock::try_read"] # [doc = " [`try_write`]: crate::sync::RwLock::try_write"] # [doc = " [`Mutex`]: crate::sync::Mutex"] # [doc = " [`RwLock`]: crate::sync::RwLock"] # [stable (feature = "rust1" , since = "1.0.0")] pub enum TryLockError < T > { # [doc = " The lock could not be acquired because another thread failed while holding"] # [doc = " the lock."] # [stable (feature = "rust1" , since = "1.0.0")] Poisoned (# [stable (feature = "rust1" , since = "1.0.0")] PoisonError < T >) , # [doc = " The lock could not be acquired at this time because the operation would"] # [doc = " otherwise block."] # [stable (feature = "rust1" , since = "1.0.0")] WouldBlock , }}}
mkitem!{# [doc = " A type alias for the result of a lock method which can be poisoned."] # [doc = ""] # [doc = " The [`Ok`] variant of this result indicates that the primitive was not"] # [doc = " poisoned, and the operation result is contained within. The [`Err`] variant indicates"] # [doc = " that the primitive was poisoned. Note that the [`Err`] variant *also* carries"] # [doc = " an associated value assigned by the lock method, and it can be acquired through the"] # [doc = " [`into_inner`] method. The semantics of the associated value depends on the corresponding"] # [doc = " lock method."] # [doc = ""] # [doc = " [`into_inner`]: PoisonError::into_inner"] # [stable (feature = "rust1" , since = "1.0.0")] pub type LockResult < T > = Result < T , PoisonError < T > > ;}
mkitem!{# [doc = " A type alias for the result of a nonblocking locking method."] # [doc = ""] # [doc = " For more information, see [`LockResult`]. A `TryLockResult` doesn't"] # [doc = " necessarily hold the associated guard in the [`Err`] type as the lock might not"] # [doc = " have been acquired for other reasons."] # [stable (feature = "rust1" , since = "1.0.0")] pub type TryLockResult < Guard > = Result < Guard , TryLockError < Guard > > ;}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < T > fmt :: Debug for PoisonError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("PoisonError") . finish_non_exhaustive () } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < T > fmt :: Display for PoisonError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "poisoned lock: another task failed inside" . fmt (f) } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < T > Error for PoisonError < T > { }}}
mkitem!{mkimpl!{impl < T > PoisonError < T > { # [doc = " Creates a `PoisonError`."] # [doc = ""] # [doc = " This is generally created by methods like [`Mutex::lock`](crate::sync::Mutex::lock)"] # [doc = " or [`RwLock::read`](crate::sync::RwLock::read)."] # [doc = ""] # [doc = " This method may panic if std was built with `panic=\"abort\"`."] # [cfg (panic = "unwind")] # [stable (feature = "sync_poison" , since = "1.2.0")] pub fn new (data : T) -> PoisonError < T > { PoisonError { data } } # [doc = " Creates a `PoisonError`."] # [doc = ""] # [doc = " This is generally created by methods like [`Mutex::lock`](crate::sync::Mutex::lock)"] # [doc = " or [`RwLock::read`](crate::sync::RwLock::read)."] # [doc = ""] # [doc = " This method may panic if std was built with `panic=\"abort\"`."] # [cfg (not (panic = "unwind"))] # [stable (feature = "sync_poison" , since = "1.2.0")] # [track_caller] pub fn new (_data : T) -> PoisonError < T > { panic ! ("PoisonError created in a libstd built with panic=\"abort\"") } # [doc = " Consumes this error indicating that a lock is poisoned, returning the"] # [doc = " associated data."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashSet;"] # [doc = " use std::sync::{Arc, Mutex};"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let mutex = Arc::new(Mutex::new(HashSet::new()));"] # [doc = ""] # [doc = " // poison the mutex"] # [doc = " let c_mutex = Arc::clone(&mutex);"] # [doc = " let _ = thread::spawn(move || {"] # [doc = "     let mut data = c_mutex.lock().unwrap();"] # [doc = "     data.insert(10);"] # [doc = "     panic!();"] # [doc = " }).join();"] # [doc = ""] # [doc = " let p_err = mutex.lock().unwrap_err();"] # [doc = " let data = p_err.into_inner();"] # [doc = " println!(\"recovered {} items\", data.len());"] # [doc = " ```"] # [stable (feature = "sync_poison" , since = "1.2.0")] pub fn into_inner (self) -> T { self . data } # [doc = " Reaches into this error indicating that a lock is poisoned, returning a"] # [doc = " reference to the associated data."] # [stable (feature = "sync_poison" , since = "1.2.0")] pub fn get_ref (& self) -> & T { & self . data } # [doc = " Reaches into this error indicating that a lock is poisoned, returning a"] # [doc = " mutable reference to the associated data."] # [stable (feature = "sync_poison" , since = "1.2.0")] pub fn get_mut (& mut self) -> & mut T { & mut self . data } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < T > From < PoisonError < T > > for TryLockError < T > { fn from (err : PoisonError < T >) -> TryLockError < T > { TryLockError :: Poisoned (err) } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < T > fmt :: Debug for TryLockError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { # [cfg (panic = "unwind")] TryLockError :: Poisoned (..) => "Poisoned(..)" . fmt (f) , # [cfg (not (panic = "unwind"))] TryLockError :: Poisoned (ref p) => match p . _never { } , TryLockError :: WouldBlock => "WouldBlock" . fmt (f) , } } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < T > fmt :: Display for TryLockError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { # [cfg (panic = "unwind")] TryLockError :: Poisoned (..) => "poisoned lock: another task failed inside" , # [cfg (not (panic = "unwind"))] TryLockError :: Poisoned (ref p) => match p . _never { } , TryLockError :: WouldBlock => "try_lock failed because the operation would block" , } . fmt (f) } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < T > Error for TryLockError < T > { # [allow (deprecated)] fn cause (& self) -> Option < & dyn Error > { match * self { # [cfg (panic = "unwind")] TryLockError :: Poisoned (ref p) => Some (p) , # [cfg (not (panic = "unwind"))] TryLockError :: Poisoned (ref p) => match p . _never { } , _ => None , } } }}}

macro_rules! map_result_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function map_result in module {}", module_path!());
    };
}

mkfn!{
    map_result_introspect!();
    pub (crate) fn map_result < T , U , F > (result : LockResult < T > , f : F) -> LockResult < U > where F : FnOnce (T) -> U , { match result { Ok (t) => Ok (f (t)) , # [cfg (panic = "unwind")] Err (PoisonError { data }) => Err (PoisonError :: new (f (data))) , } }
}