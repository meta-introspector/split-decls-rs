mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{use crate :: cell :: UnsafeCell ;}
mkuse!{use crate :: hint ;}
mkuse!{use crate :: ops :: { Deref , DerefMut } ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicBool , Ordering } ;}
mkitem!{mkstruct!{# [derive (Default)] pub struct SpinMutex < T > { value : UnsafeCell < T > , lock : Atomic < bool > , }}}
mkitem!{mkimpl!{unsafe impl < T : Send > Send for SpinMutex < T > { }}}
mkitem!{mkimpl!{unsafe impl < T : Send > Sync for SpinMutex < T > { }}}
mkitem!{mkstruct!{pub struct SpinMutexGuard < 'a , T : 'a > { mutex : & 'a SpinMutex < T > , }}}
mkitem!{mkimpl!{impl < 'a , T > ! Send for SpinMutexGuard < 'a , T > { }}}
mkitem!{mkimpl!{unsafe impl < 'a , T : Sync > Sync for SpinMutexGuard < 'a , T > { }}}
mkitem!{mkimpl!{impl < T > SpinMutex < T > { pub const fn new (value : T) -> Self { SpinMutex { value : UnsafeCell :: new (value) , lock : AtomicBool :: new (false) } } # [inline (always)] pub fn lock (& self) -> SpinMutexGuard < '_ , T > { loop { match self . try_lock () { None => { while self . lock . load (Ordering :: Relaxed) { hint :: spin_loop () } } Some (guard) => return guard , } } } # [inline (always)] pub fn try_lock (& self) -> Option < SpinMutexGuard < '_ , T > > { if self . lock . compare_exchange (false , true , Ordering :: Acquire , Ordering :: Acquire) . is_ok () { Some (SpinMutexGuard { mutex : self }) } else { None } } }}}
mkitem!{# [doc = " Lock the Mutex or return false."] pub macro try_lock_or_false ($ e : expr) { if let Some (v) = $ e . try_lock () { v } else { return false } }}
mkitem!{mkimpl!{impl < 'a , T > Deref for SpinMutexGuard < 'a , T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . mutex . value . get () } } }}}
mkitem!{mkimpl!{impl < 'a , T > DerefMut for SpinMutexGuard < 'a , T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . mutex . value . get () } } }}}
mkitem!{mkimpl!{impl < 'a , T > Drop for SpinMutexGuard < 'a , T > { fn drop (& mut self) { self . mutex . lock . store (false , Ordering :: Release) } }}}