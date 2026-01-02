mkuse!{use core :: ffi :: c_void ;}
mkuse!{use crate :: pin :: Pin ;}
mkuse!{use crate :: sync :: atomic :: Ordering :: { Acquire , Release } ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicI8 } ;}
mkuse!{use crate :: sys :: { c , dur2timeout } ;}
mkuse!{use crate :: time :: Duration ;}
mkitem!{mkstruct!{pub struct Parker { state : Atomic < i8 > , }}}
mkitem!{const PARKED : i8 = - 1 ;}
mkitem!{const EMPTY : i8 = 0 ;}
mkitem!{const NOTIFIED : i8 = 1 ;}
mkitem!{mkimpl!{impl Parker { # [doc = " Constructs the Windows parker. The UNIX parker implementation"] # [doc = " requires this to happen in-place."] pub unsafe fn new_in_place (parker : * mut Parker) { parker . write (Self { state : AtomicI8 :: new (EMPTY) }) ; } pub unsafe fn park (self : Pin < & Self >) { if self . state . fetch_sub (1 , Acquire) == NOTIFIED { return ; } # [cfg (target_vendor = "win7")] if c :: WaitOnAddress :: option () . is_none () { return keyed_events :: park (self) ; } loop { c :: WaitOnAddress (self . ptr () , & PARKED as * const _ as * const c_void , 1 , c :: INFINITE) ; if self . state . compare_exchange (NOTIFIED , EMPTY , Acquire , Acquire) . is_ok () { return ; } else { } } } pub unsafe fn park_timeout (self : Pin < & Self > , timeout : Duration) { if self . state . fetch_sub (1 , Acquire) == NOTIFIED { return ; } # [cfg (target_vendor = "win7")] if c :: WaitOnAddress :: option () . is_none () { return keyed_events :: park_timeout (self , timeout) ; } c :: WaitOnAddress (self . ptr () , & PARKED as * const _ as * const c_void , 1 , dur2timeout (timeout)) ; if self . state . swap (EMPTY , Acquire) == NOTIFIED { } else { } } pub fn unpark (self : Pin < & Self >) { if self . state . swap (NOTIFIED , Release) == PARKED { unsafe { # [cfg (target_vendor = "win7")] if c :: WakeByAddressSingle :: option () . is_none () { return keyed_events :: unpark (self) ; } c :: WakeByAddressSingle (self . ptr ()) ; } } } fn ptr (& self) -> * const c_void { (& raw const self . state) . cast :: < c_void > () } }}}
mkmod!{keyed_events, { 
                getname!(keyed_events);
                getsrc!(keyed_events);
                getpath!(keyed_events);
                get_deps!(keyed_events);
                get_crates!(keyed_events);
                mkinclude!(keyed_events);
                mkuse!{use core :: pin :: Pin ;}
mkuse!{use core :: ptr ;}
mkuse!{use core :: sync :: atomic :: Ordering :: { Acquire , Relaxed } ;}
mkuse!{use core :: sync :: atomic :: { Atomic , AtomicPtr } ;}
mkuse!{use core :: time :: Duration ;}
mkuse!{use super :: { EMPTY , NOTIFIED , Parker } ;}
mkuse!{use crate :: sys :: c ;}

macro_rules! park_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function park in module {}", module_path!());
    };
}

mkfn!{
    park_introspect!();
    pub unsafe fn park (parker : Pin < & Parker >) { c :: NtWaitForKeyedEvent (keyed_event_handle () , parker . ptr () , false , ptr :: null_mut ()) ; parker . state . swap (EMPTY , Acquire) ; return ; }
}

macro_rules! park_timeout_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function park_timeout in module {}", module_path!());
    };
}

mkfn!{
    park_timeout_introspect!();
    pub unsafe fn park_timeout (parker : Pin < & Parker > , timeout : Duration) { let handle = keyed_event_handle () ; let mut timeout = match i64 :: try_from ((timeout . as_nanos () + 99) / 100) { Ok (t) => - t , Err (_) => i64 :: MIN , } ; let unparked = c :: NtWaitForKeyedEvent (handle , parker . ptr () , false , & mut timeout) == c :: STATUS_SUCCESS ; let prev_state = parker . state . swap (EMPTY , Acquire) ; if ! unparked && prev_state == NOTIFIED { c :: NtWaitForKeyedEvent (handle , parker . ptr () , false , ptr :: null_mut ()) ; } }
}

macro_rules! unpark_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unpark in module {}", module_path!());
    };
}

mkfn!{
    unpark_introspect!();
    pub unsafe fn unpark (parker : Pin < & Parker >) { c :: NtReleaseKeyedEvent (keyed_event_handle () , parker . ptr () , false , ptr :: null_mut ()) ; }
}

macro_rules! keyed_event_handle_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function keyed_event_handle in module {}", module_path!());
    };
}

mkfn!{
    keyed_event_handle_introspect!();
    fn keyed_event_handle () -> c :: HANDLE { const INVALID : c :: HANDLE = ptr :: without_provenance_mut (! 0) ; static HANDLE : Atomic < * mut crate :: ffi :: c_void > = AtomicPtr :: new (INVALID) ; match HANDLE . load (Relaxed) { INVALID => { let mut handle = c :: INVALID_HANDLE_VALUE ; unsafe { match c :: NtCreateKeyedEvent (& mut handle , c :: GENERIC_READ | c :: GENERIC_WRITE , ptr :: null_mut () , 0 ,) { c :: STATUS_SUCCESS => { } r => panic ! ("Unable to create keyed event handle: error {r}") , } } match HANDLE . compare_exchange (INVALID , handle , Relaxed , Relaxed) { Ok (_) => handle , Err (h) => { unsafe { c :: CloseHandle (handle) ; } h } } } handle => handle , } }
} 
            }}