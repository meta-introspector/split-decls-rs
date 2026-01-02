mkuse!{use super :: hermit_abi ;}
mkuse!{use crate :: ptr :: null ;}
mkuse!{use crate :: sync :: atomic :: Atomic ;}
mkuse!{use crate :: time :: Duration ;}
mkitem!{# [doc = " An atomic for use as a futex that is at least 32-bits but may be larger"] pub type Futex = Atomic < Primitive > ;}
mkitem!{# [doc = " Must be the underlying type of Futex"] pub type Primitive = u32 ;}
mkitem!{# [doc = " An atomic for use as a futex that is at least 8-bits but may be larger."] pub type SmallFutex = Atomic < SmallPrimitive > ;}
mkitem!{# [doc = " Must be the underlying type of SmallFutex"] pub type SmallPrimitive = u32 ;}

macro_rules! futex_wait_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function futex_wait in module {}", module_path!());
    };
}

mkfn!{
    futex_wait_introspect!();
    pub fn futex_wait (futex : & Atomic < u32 > , expected : u32 , timeout : Option < Duration >) -> bool { let timespec = timeout . and_then (| dur | { Some (hermit_abi :: timespec { tv_sec : dur . as_secs () . try_into () . ok () ? , tv_nsec : dur . subsec_nanos () . try_into () . ok () ? , }) }) ; let r = unsafe { hermit_abi :: futex_wait (futex . as_ptr () , expected , timespec . as_ref () . map_or (null () , | t | t as * const hermit_abi :: timespec) , hermit_abi :: FUTEX_RELATIVE_TIMEOUT ,) } ; r != - hermit_abi :: errno :: ETIMEDOUT }
}

macro_rules! futex_wake_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function futex_wake in module {}", module_path!());
    };
}

mkfn!{
    futex_wake_introspect!();
    # [inline] pub fn futex_wake (futex : & Atomic < u32 >) -> bool { unsafe { hermit_abi :: futex_wake (futex . as_ptr () , 1) > 0 } }
}

macro_rules! futex_wake_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function futex_wake_all in module {}", module_path!());
    };
}

mkfn!{
    futex_wake_all_introspect!();
    # [inline] pub fn futex_wake_all (futex : & Atomic < u32 >) { unsafe { hermit_abi :: futex_wake (futex . as_ptr () , i32 :: MAX) ; } }
}