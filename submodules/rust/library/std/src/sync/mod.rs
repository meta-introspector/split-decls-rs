mkuse!{# [unstable (feature = "exclusive_wrapper" , issue = "98407")] pub use core :: sync :: Exclusive ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: sync :: atomic ;}
mkuse!{# [unstable (feature = "unique_rc_arc" , issue = "112566")] pub use alloc_crate :: sync :: UniqueArc ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use alloc_crate :: sync :: { Arc , Weak } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: barrier :: { Barrier , BarrierWaitResult } ;}
mkuse!{# [stable (feature = "lazy_cell" , since = "1.80.0")] pub use self :: lazy_lock :: LazyLock ;}
mkuse!{# [stable (feature = "once_cell" , since = "1.70.0")] pub use self :: once_lock :: OnceLock ;}
mkuse!{# [unstable (feature = "reentrant_lock" , issue = "121440")] pub use self :: reentrant_lock :: { ReentrantLock , ReentrantLockGuard } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (inline)] pub use self :: poison :: { LockResult , PoisonError } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (inline)] pub use self :: poison :: { Mutex , MutexGuard , TryLockError , TryLockResult , Condvar , Once , OnceState , RwLock , RwLockReadGuard , RwLockWriteGuard , } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] # [doc (inline)] # [expect (deprecated)] pub use self :: poison :: ONCE_INIT ;}
mkuse!{# [unstable (feature = "mapped_lock_guards" , issue = "117108")] # [doc (inline)] pub use self :: poison :: { MappedMutexGuard , MappedRwLockReadGuard , MappedRwLockWriteGuard } ;}
mkmod!{mpmc, { 
                getname!(mpmc);
                getsrc!(mpmc);
                getpath!(mpmc);
                get_deps!(mpmc);
                get_crates!(mpmc);
                mkinclude!(mpmc);
                 
            }}
mkmod!{mpsc, { 
                getname!(mpsc);
                getsrc!(mpsc);
                getpath!(mpsc);
                get_deps!(mpsc);
                get_crates!(mpsc);
                mkinclude!(mpsc);
                 
            }}
mkmod!{nonpoison, { 
                getname!(nonpoison);
                getsrc!(nonpoison);
                getpath!(nonpoison);
                get_deps!(nonpoison);
                get_crates!(nonpoison);
                mkinclude!(nonpoison);
                 
            }}
mkmod!{poison, { 
                getname!(poison);
                getsrc!(poison);
                getpath!(poison);
                get_deps!(poison);
                get_crates!(poison);
                mkinclude!(poison);
                 
            }}
mkmod!{barrier, { 
                getname!(barrier);
                getsrc!(barrier);
                getpath!(barrier);
                get_deps!(barrier);
                get_crates!(barrier);
                mkinclude!(barrier);
                 
            }}
mkmod!{lazy_lock, { 
                getname!(lazy_lock);
                getsrc!(lazy_lock);
                getpath!(lazy_lock);
                get_deps!(lazy_lock);
                get_crates!(lazy_lock);
                mkinclude!(lazy_lock);
                 
            }}
mkmod!{once_lock, { 
                getname!(once_lock);
                getsrc!(once_lock);
                getpath!(once_lock);
                get_deps!(once_lock);
                get_crates!(once_lock);
                mkinclude!(once_lock);
                 
            }}
mkmod!{reentrant_lock, { 
                getname!(reentrant_lock);
                getsrc!(reentrant_lock);
                getpath!(reentrant_lock);
                get_deps!(reentrant_lock);
                get_crates!(reentrant_lock);
                mkinclude!(reentrant_lock);
                 
            }}
mkitem!{mkstruct!{# [doc = " A type indicating whether a timed wait on a condition variable returned"] # [doc = " due to a time out or not."] # [doc = ""] # [doc = " It is returned by the [`wait_timeout`] method."] # [doc = ""] # [doc = " [`wait_timeout`]: Condvar::wait_timeout"] # [derive (Debug , PartialEq , Eq , Copy , Clone)] # [stable (feature = "wait_timeout" , since = "1.5.0")] pub struct WaitTimeoutResult (bool) ;}}
mkitem!{mkimpl!{impl WaitTimeoutResult { # [doc = " Returns `true` if the wait was known to have timed out."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " This example spawns a thread which will sleep 20 milliseconds before"] # [doc = " updating a boolean value and then notifying the condvar."] # [doc = ""] # [doc = " The main thread will wait with a 10 millisecond timeout on the condvar"] # [doc = " and will leave the loop upon timeout."] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::{Arc, Condvar, Mutex};"] # [doc = " use std::thread;"] # [doc = " use std::time::Duration;"] # [doc = ""] # [doc = " let pair = Arc::new((Mutex::new(false), Condvar::new()));"] # [doc = " let pair2 = Arc::clone(&pair);"] # [doc = ""] # [doc = " # let handle ="] # [doc = " thread::spawn(move || {"] # [doc = "     let (lock, cvar) = &*pair2;"] # [doc = ""] # [doc = "     // Let's wait 20 milliseconds before notifying the condvar."] # [doc = "     thread::sleep(Duration::from_millis(20));"] # [doc = ""] # [doc = "     let mut started = lock.lock().unwrap();"] # [doc = "     // We update the boolean value."] # [doc = "     *started = true;"] # [doc = "     cvar.notify_one();"] # [doc = " });"] # [doc = ""] # [doc = " // Wait for the thread to start up."] # [doc = " let (lock, cvar) = &*pair;"] # [doc = " loop {"] # [doc = "     // Let's put a timeout on the condvar's wait."] # [doc = "     let result = cvar.wait_timeout(lock.lock().unwrap(), Duration::from_millis(10)).unwrap();"] # [doc = "     // 10 milliseconds have passed."] # [doc = "     if result.1.timed_out() {"] # [doc = "         // timed out now and we can leave."] # [doc = "         break"] # [doc = "     }"] # [doc = " }"] # [doc = " # // Prevent leaks for Miri."] # [doc = " # let _ = handle.join();"] # [doc = " ```"] # [must_use] # [stable (feature = "wait_timeout" , since = "1.5.0")] pub fn timed_out (& self) -> bool { self . 0 } }}}