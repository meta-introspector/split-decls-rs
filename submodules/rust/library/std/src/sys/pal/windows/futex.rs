mkuse!{use core :: ffi :: c_void ;}
mkuse!{use core :: ptr ;}
mkuse!{use core :: sync :: atomic :: { Atomic , AtomicBool , AtomicI8 , AtomicI16 , AtomicI32 , AtomicI64 , AtomicIsize , AtomicPtr , AtomicU8 , AtomicU16 , AtomicU32 , AtomicU64 , AtomicUsize , } ;}
mkuse!{use core :: time :: Duration ;}
mkuse!{use super :: api :: { self , WinError } ;}
mkuse!{use crate :: sys :: { c , dur2timeout } ;}
mkitem!{# [doc = " An atomic for use as a futex that is at least 32-bits but may be larger"] pub type Futex = Atomic < Primitive > ;}
mkitem!{# [doc = " Must be the underlying type of Futex"] pub type Primitive = u32 ;}
mkitem!{# [doc = " An atomic for use as a futex that is at least 8-bits but may be larger."] pub type SmallFutex = Atomic < SmallPrimitive > ;}
mkitem!{# [doc = " Must be the underlying type of SmallFutex"] pub type SmallPrimitive = u8 ;}
mkitem!{mktrait!{pub unsafe trait Futexable { }}}
mkitem!{mktrait!{pub unsafe trait Waitable { type Futex ; }}}
mkitem!{macro_rules ! unsafe_waitable_int { ($ (($ int : ty , $ atomic : ty)) ,*$ (,) ?) => { $ (unsafe impl Waitable for $ int { type Futex = $ atomic ; } unsafe impl Futexable for $ atomic { }) * } ; }}
mkitem!{unsafe_waitable_int ! { (bool , AtomicBool) , (i8 , AtomicI8) , (i16 , AtomicI16) , (i32 , AtomicI32) , (i64 , AtomicI64) , (isize , AtomicIsize) , (u8 , AtomicU8) , (u16 , AtomicU16) , (u32 , AtomicU32) , (u64 , AtomicU64) , (usize , AtomicUsize) , }}
mkitem!{mkimpl!{unsafe impl < T > Waitable for * const T { type Futex = Atomic < * mut T > ; }}}
mkitem!{mkimpl!{unsafe impl < T > Waitable for * mut T { type Futex = Atomic < * mut T > ; }}}
mkitem!{mkimpl!{unsafe impl < T > Futexable for AtomicPtr < T > { }}}

macro_rules! wait_on_address_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function wait_on_address in module {}", module_path!());
    };
}

mkfn!{
    wait_on_address_introspect!();
    pub fn wait_on_address < W : Waitable > (address : & W :: Futex , compare : W , timeout : Option < Duration > ,) -> bool { unsafe { let addr = ptr :: from_ref (address) . cast :: < c_void > () ; let size = size_of :: < W > () ; let compare_addr = (& raw const compare) . cast :: < c_void > () ; let timeout = timeout . map (dur2timeout) . unwrap_or (c :: INFINITE) ; c :: WaitOnAddress (addr , compare_addr , size , timeout) == c :: TRUE } }
}

macro_rules! wake_by_address_single_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function wake_by_address_single in module {}", module_path!());
    };
}

mkfn!{
    wake_by_address_single_introspect!();
    pub fn wake_by_address_single < T : Futexable > (address : & T) { unsafe { let addr = ptr :: from_ref (address) . cast :: < c_void > () ; c :: WakeByAddressSingle (addr) ; } }
}

macro_rules! wake_by_address_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function wake_by_address_all in module {}", module_path!());
    };
}

mkfn!{
    wake_by_address_all_introspect!();
    pub fn wake_by_address_all < T : Futexable > (address : & T) { unsafe { let addr = ptr :: from_ref (address) . cast :: < c_void > () ; c :: WakeByAddressAll (addr) ; } }
}

macro_rules! futex_wait_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function futex_wait in module {}", module_path!());
    };
}

mkfn!{
    futex_wait_introspect!();
    pub fn futex_wait < W : Waitable > (futex : & W :: Futex , expected : W , timeout : Option < Duration >) -> bool { wait_on_address (futex , expected , timeout) || api :: get_last_error () != WinError :: TIMEOUT }
}

macro_rules! futex_wake_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function futex_wake in module {}", module_path!());
    };
}

mkfn!{
    futex_wake_introspect!();
    pub fn futex_wake < T : Futexable > (futex : & T) -> bool { wake_by_address_single (futex) ; false }
}

macro_rules! futex_wake_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function futex_wake_all in module {}", module_path!());
    };
}

mkfn!{
    futex_wake_all_introspect!();
    pub fn futex_wake_all < T : Futexable > (futex : & T) { wake_by_address_all (futex) }
}