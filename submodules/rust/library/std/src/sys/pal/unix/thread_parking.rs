mkuse!{use libc :: { _lwp_self , CLOCK_MONOTONIC , c_long , clockid_t , lwpid_t , time_t , timespec } ;}
mkuse!{use crate :: ffi :: { c_int , c_void } ;}
mkuse!{use crate :: ptr ;}
mkuse!{use crate :: time :: Duration ;}
mkitem!{unsafe extern "C" { fn ___lwp_park60 (clock_id : clockid_t , flags : c_int , ts : * mut timespec , unpark : lwpid_t , hint : * const c_void , unparkhint : * const c_void ,) -> c_int ; fn _lwp_unpark (lwp : lwpid_t , hint : * const c_void) -> c_int ; }}
mkitem!{pub type ThreadId = lwpid_t ;}

macro_rules! current_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current in module {}", module_path!());
    };
}

mkfn!{
    current_introspect!();
    # [inline] pub fn current () -> ThreadId { unsafe { _lwp_self () } }
}

macro_rules! park_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function park in module {}", module_path!());
    };
}

mkfn!{
    park_introspect!();
    # [inline] pub fn park (hint : usize) { unsafe { ___lwp_park60 (0 , 0 , ptr :: null_mut () , 0 , ptr :: without_provenance (hint) , ptr :: null ()) ; } }
}

macro_rules! park_timeout_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function park_timeout in module {}", module_path!());
    };
}

mkfn!{
    park_timeout_introspect!();
    pub fn park_timeout (dur : Duration , hint : usize) { let mut timeout = timespec { tv_sec : dur . as_secs () . try_into () . ok () . unwrap_or (time_t :: MAX) , tv_nsec : dur . subsec_nanos () as c_long , } ; unsafe { ___lwp_park60 (CLOCK_MONOTONIC , 0 , & mut timeout , 0 , ptr :: without_provenance (hint) , ptr :: null () ,) ; } }
}

macro_rules! unpark_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unpark in module {}", module_path!());
    };
}

mkfn!{
    unpark_introspect!();
    # [inline] pub fn unpark (tid : ThreadId , hint : usize) { unsafe { _lwp_unpark (tid , ptr :: without_provenance (hint)) ; } }
}