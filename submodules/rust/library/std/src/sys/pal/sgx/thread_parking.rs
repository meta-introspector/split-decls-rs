mkuse!{use fortanix_sgx_abi :: { EV_UNPARK , WAIT_INDEFINITE } ;}
mkuse!{use super :: abi :: usercalls ;}
mkuse!{use crate :: io :: ErrorKind ;}
mkuse!{use crate :: time :: Duration ;}
mkitem!{pub type ThreadId = fortanix_sgx_abi :: Tcs ;}
mkuse!{pub use super :: abi :: thread :: current ;}

macro_rules! park_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function park in module {}", module_path!());
    };
}

mkfn!{
    park_introspect!();
    pub fn park (_hint : usize) { usercalls :: wait (EV_UNPARK , WAIT_INDEFINITE) . unwrap () ; }
}

macro_rules! park_timeout_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function park_timeout in module {}", module_path!());
    };
}

mkfn!{
    park_timeout_introspect!();
    pub fn park_timeout (dur : Duration , _hint : usize) { let timeout = u128 :: min (dur . as_nanos () , WAIT_INDEFINITE as u128 - 1) as u64 ; if let Err (e) = usercalls :: wait (EV_UNPARK , timeout) { assert ! (matches ! (e . kind () , ErrorKind :: TimedOut | ErrorKind :: WouldBlock)) } }
}

macro_rules! unpark_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unpark in module {}", module_path!());
    };
}

mkfn!{
    unpark_introspect!();
    pub fn unpark (tid : ThreadId , _hint : usize) { let _ = usercalls :: send (EV_UNPARK , Some (tid)) ; }
}