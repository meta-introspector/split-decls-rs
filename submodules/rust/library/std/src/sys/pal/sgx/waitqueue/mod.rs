mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkmod!{spin_mutex, { 
                getname!(spin_mutex);
                getsrc!(spin_mutex);
                getpath!(spin_mutex);
                get_deps!(spin_mutex);
                get_crates!(spin_mutex);
                mkinclude!(spin_mutex);
                 
            }}
mkmod!{unsafe_list, { 
                getname!(unsafe_list);
                getsrc!(unsafe_list);
                getpath!(unsafe_list);
                get_deps!(unsafe_list);
                get_crates!(unsafe_list);
                mkinclude!(unsafe_list);
                 
            }}
mkuse!{use fortanix_sgx_abi :: { EV_UNPARK , Tcs , WAIT_INDEFINITE } ;}
mkuse!{pub use self :: spin_mutex :: { SpinMutex , SpinMutexGuard , try_lock_or_false } ;}
mkuse!{use self :: unsafe_list :: { UnsafeList , UnsafeListEntry } ;}
mkuse!{use super :: abi :: { thread , usercalls } ;}
mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: ops :: { Deref , DerefMut } ;}
mkuse!{use crate :: panic :: { self , AssertUnwindSafe } ;}
mkuse!{use crate :: time :: Duration ;}
mkitem!{mkstruct!{# [doc = " An queue entry in a `WaitQueue`."] struct WaitEntry { # [doc = " TCS address of the thread that is waiting"] tcs : Tcs , # [doc = " Whether this thread has been notified to be awoken"] wake : bool , }}}
mkitem!{mkstruct!{# [doc = " Data stored with a `WaitQueue` alongside it. This ensures accesses to the"] # [doc = " queue and the data are synchronized, since the type itself is not `Sync`."] # [doc = ""] # [doc = " Consumers of this API should use a synchronization primitive for shared"] # [doc = " access, such as `SpinMutex`."] # [derive (Default)] pub struct WaitVariable < T > { queue : WaitQueue , lock : T , }}}
mkitem!{mkimpl!{impl < T > WaitVariable < T > { pub const fn new (var : T) -> Self { WaitVariable { queue : WaitQueue :: new () , lock : var } } pub fn lock_var (& self) -> & T { & self . lock } pub fn lock_var_mut (& mut self) -> & mut T { & mut self . lock } }}}
mkitem!{mkenum!{# [derive (Copy , Clone)] pub enum NotifiedTcs { Single (Tcs) , All { _count : NonZero < usize > } , }}}
mkitem!{mkstruct!{# [doc = " An RAII guard that will notify a set of target threads as well as unlock"] # [doc = " a mutex on drop."] pub struct WaitGuard < 'a , T : 'a > { mutex_guard : Option < SpinMutexGuard < 'a , WaitVariable < T > > > , notified_tcs : NotifiedTcs , }}}
mkitem!{mkstruct!{# [doc = " A queue of threads that are waiting on some synchronization primitive."] # [doc = ""] # [doc = " `UnsafeList` entries are allocated on the waiting thread's stack. This"] # [doc = " avoids any global locking that might happen in the heap allocator. This is"] # [doc = " safe because the waiting thread will not return from that stack frame until"] # [doc = " after it is notified. The notifying thread ensures to clean up any"] # [doc = " references to the list entries before sending the wakeup event."] pub struct WaitQueue { inner : UnsafeList < SpinMutex < WaitEntry > > , }}}
mkitem!{mkimpl!{unsafe impl Send for WaitQueue { }}}
mkitem!{mkimpl!{impl Default for WaitQueue { fn default () -> Self { Self :: new () } }}}
mkitem!{mkimpl!{impl < 'a , T > Deref for WaitGuard < 'a , T > { type Target = SpinMutexGuard < 'a , WaitVariable < T > > ; fn deref (& self) -> & Self :: Target { self . mutex_guard . as_ref () . unwrap () } }}}
mkitem!{mkimpl!{impl < 'a , T > DerefMut for WaitGuard < 'a , T > { fn deref_mut (& mut self) -> & mut Self :: Target { self . mutex_guard . as_mut () . unwrap () } }}}
mkitem!{mkimpl!{impl < 'a , T > Drop for WaitGuard < 'a , T > { fn drop (& mut self) { drop (self . mutex_guard . take ()) ; let target_tcs = match self . notified_tcs { NotifiedTcs :: Single (tcs) => Some (tcs) , NotifiedTcs :: All { .. } => None , } ; rtunwrap ! (Ok , usercalls :: send (EV_UNPARK , target_tcs)) ; } }}}
mkitem!{mkimpl!{impl WaitQueue { pub const fn new () -> Self { WaitQueue { inner : UnsafeList :: new () } } # [doc = " Adds the calling thread to the `WaitVariable`'s wait queue, then wait"] # [doc = " until a wakeup event."] # [doc = ""] # [doc = " This function does not return until this thread has been awoken. When `before_wait` panics,"] # [doc = " this function will abort."] pub fn wait < T , F : FnOnce () > (mut guard : SpinMutexGuard < '_ , WaitVariable < T > > , before_wait : F) { unsafe { let mut entry = UnsafeListEntry :: new (SpinMutex :: new (WaitEntry { tcs : thread :: current () , wake : false , })) ; let entry = guard . queue . inner . push (& mut entry) ; drop (guard) ; if let Err (_e) = panic :: catch_unwind (AssertUnwindSafe (| | before_wait ())) { rtabort ! ("Panic before wait on wakeup event") } while ! entry . lock () . wake { let eventset = rtunwrap ! (Ok , usercalls :: wait (EV_UNPARK , WAIT_INDEFINITE)) ; rtassert ! (eventset & EV_UNPARK == EV_UNPARK) ; } } } # [doc = " Adds the calling thread to the `WaitVariable`'s wait queue, then wait"] # [doc = " until a wakeup event or timeout. If event was observed, returns true."] # [doc = " If not, it will remove the calling thread from the wait queue."] # [doc = " When `before_wait` panics, this function will abort."] pub fn wait_timeout < T , F : FnOnce () > (lock : & SpinMutex < WaitVariable < T > > , timeout : Duration , before_wait : F ,) -> bool { unsafe { let mut entry = UnsafeListEntry :: new (SpinMutex :: new (WaitEntry { tcs : thread :: current () , wake : false , })) ; let entry_lock = lock . lock () . queue . inner . push (& mut entry) ; if let Err (_e) = panic :: catch_unwind (AssertUnwindSafe (| | before_wait ())) { rtabort ! ("Panic before wait on wakeup event or timeout") } usercalls :: wait_timeout (EV_UNPARK , timeout , | | entry_lock . lock () . wake) ; let mut guard = lock . lock () ; let success = entry_lock . lock () . wake ; if ! success { guard . queue . inner . remove (& mut entry) ; } success } } # [doc = " Either find the next waiter on the wait queue, or return the mutex"] # [doc = " guard unchanged."] # [doc = ""] # [doc = " If a waiter is found, a `WaitGuard` is returned which will notify the"] # [doc = " waiter when it is dropped."] pub fn notify_one < T > (mut guard : SpinMutexGuard < '_ , WaitVariable < T > > ,) -> Result < WaitGuard < '_ , T > , SpinMutexGuard < '_ , WaitVariable < T > > > { unsafe { let tcs = guard . queue . inner . pop () . map (| entry | -> Tcs { let mut entry_guard = entry . lock () ; entry_guard . wake = true ; entry_guard . tcs }) ; if let Some (tcs) = tcs { Ok (WaitGuard { mutex_guard : Some (guard) , notified_tcs : NotifiedTcs :: Single (tcs) }) } else { Err (guard) } } } # [doc = " Either find any and all waiters on the wait queue, or return the mutex"] # [doc = " guard unchanged."] # [doc = ""] # [doc = " If at least one waiter is found, a `WaitGuard` is returned which will"] # [doc = " notify all waiters when it is dropped."] pub fn notify_all < T > (mut guard : SpinMutexGuard < '_ , WaitVariable < T > > ,) -> Result < WaitGuard < '_ , T > , SpinMutexGuard < '_ , WaitVariable < T > > > { unsafe { let mut count = 0 ; while let Some (entry) = guard . queue . inner . pop () { count += 1 ; let mut entry_guard = entry . lock () ; entry_guard . wake = true ; } if let Some (count) = NonZero :: new (count) { Ok (WaitGuard { mutex_guard : Some (guard) , notified_tcs : NotifiedTcs :: All { _count : count } , }) } else { Err (guard) } } } }}}