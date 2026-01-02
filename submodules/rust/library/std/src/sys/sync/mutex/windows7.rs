mkuse!{use crate :: cell :: UnsafeCell ;}
mkuse!{use crate :: sys :: c ;}
mkitem!{mkstruct!{pub struct Mutex { srwlock : UnsafeCell < c :: SRWLOCK > , }}}
mkitem!{mkimpl!{unsafe impl Send for Mutex { }}}
mkitem!{mkimpl!{unsafe impl Sync for Mutex { }}}

macro_rules! raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function raw in module {}", module_path!());
    };
}

mkfn!{
    raw_introspect!();
    # [inline] pub unsafe fn raw (m : & Mutex) -> * mut c :: SRWLOCK { m . srwlock . get () }
}
mkitem!{mkimpl!{impl Mutex { # [inline] pub const fn new () -> Mutex { Mutex { srwlock : UnsafeCell :: new (c :: SRWLOCK_INIT) } } # [inline] pub fn lock (& self) { unsafe { c :: AcquireSRWLockExclusive (raw (self)) ; } } # [inline] pub fn try_lock (& self) -> bool { unsafe { c :: TryAcquireSRWLockExclusive (raw (self)) } } # [inline] pub unsafe fn unlock (& self) { c :: ReleaseSRWLockExclusive (raw (self)) ; } }}}