mkuse!{use crate :: fmt ;}
mkitem!{# [doc = " A type alias for the result of a nonblocking locking method."] # [unstable (feature = "sync_nonpoison" , issue = "134645")] pub type TryLockResult < Guard > = Result < Guard , WouldBlock > ;}
mkitem!{mkstruct!{# [doc = " A lock could not be acquired at this time because the operation would otherwise block."] # [unstable (feature = "sync_nonpoison" , issue = "134645")] pub struct WouldBlock ;}}
mkitem!{mkimpl!{# [unstable (feature = "sync_nonpoison" , issue = "134645")] impl fmt :: Debug for WouldBlock { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "WouldBlock" . fmt (f) } }}}
mkitem!{mkimpl!{# [unstable (feature = "sync_nonpoison" , issue = "134645")] impl fmt :: Display for WouldBlock { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "try_lock failed because the operation would block" . fmt (f) } }}}
mkuse!{# [unstable (feature = "nonpoison_condvar" , issue = "134645")] pub use self :: condvar :: Condvar ;}
mkuse!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] pub use self :: mutex :: MappedMutexGuard ;}
mkuse!{# [unstable (feature = "nonpoison_mutex" , issue = "134645")] pub use self :: mutex :: { Mutex , MutexGuard } ;}
mkuse!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] pub use self :: rwlock :: { MappedRwLockReadGuard , MappedRwLockWriteGuard } ;}
mkuse!{# [unstable (feature = "nonpoison_rwlock" , issue = "134645")] pub use self :: rwlock :: { RwLock , RwLockReadGuard , RwLockWriteGuard } ;}
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
mkmod!{rwlock, { 
                getname!(rwlock);
                getsrc!(rwlock);
                getpath!(rwlock);
                get_deps!(rwlock);
                get_crates!(rwlock);
                mkinclude!(rwlock);
                 
            }}