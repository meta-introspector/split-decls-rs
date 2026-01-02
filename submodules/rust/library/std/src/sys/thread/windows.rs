mkuse!{use core :: ffi :: c_void ;}
mkuse!{use crate :: ffi :: CStr ;}
mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: os :: windows :: io :: { AsRawHandle , HandleOrNull } ;}
mkuse!{use crate :: sys :: handle :: Handle ;}
mkuse!{use crate :: sys :: pal :: time :: WaitableTimer ;}
mkuse!{use crate :: sys :: pal :: { dur2timeout , to_u16s } ;}
mkuse!{use crate :: sys :: { c , stack_overflow } ;}
mkuse!{use crate :: sys_common :: FromInner ;}
mkuse!{use crate :: time :: Duration ;}
mkuse!{use crate :: { io , ptr } ;}
mkitem!{pub const DEFAULT_MIN_STACK_SIZE : usize = 2 * 1024 * 1024 ;}
mkitem!{mkstruct!{pub struct Thread { handle : Handle , }}}
mkitem!{mkimpl!{impl Thread { # [cfg_attr (miri , track_caller)] pub unsafe fn new (stack : usize , _name : Option < & str > , p : Box < dyn FnOnce () > ,) -> io :: Result < Thread > { let p = Box :: into_raw (Box :: new (p)) ; let ret = unsafe { let ret = c :: CreateThread (ptr :: null_mut () , stack , Some (thread_start) , p as * mut _ , c :: STACK_SIZE_PARAM_IS_A_RESERVATION , ptr :: null_mut () ,) ; HandleOrNull :: from_raw_handle (ret) } ; return if let Ok (handle) = ret . try_into () { Ok (Thread { handle : Handle :: from_inner (handle) }) } else { unsafe { drop (Box :: from_raw (p)) } ; Err (io :: Error :: last_os_error ()) } ; unsafe extern "system" fn thread_start (main : * mut c_void) -> u32 { stack_overflow :: reserve_stack () ; unsafe { Box :: from_raw (main as * mut Box < dyn FnOnce () >) () } ; 0 } } pub fn join (self) { let rc = unsafe { c :: WaitForSingleObject (self . handle . as_raw_handle () , c :: INFINITE) } ; if rc == c :: WAIT_FAILED { panic ! ("failed to join on thread: {}" , io :: Error :: last_os_error ()) ; } } pub fn handle (& self) -> & Handle { & self . handle } pub fn into_handle (self) -> Handle { self . handle } }}}

macro_rules! available_parallelism_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function available_parallelism in module {}", module_path!());
    };
}

mkfn!{
    available_parallelism_introspect!();
    pub fn available_parallelism () -> io :: Result < NonZero < usize > > { let res = unsafe { let mut sysinfo : c :: SYSTEM_INFO = crate :: mem :: zeroed () ; c :: GetSystemInfo (& mut sysinfo) ; sysinfo . dwNumberOfProcessors as usize } ; match res { 0 => Err (io :: Error :: UNKNOWN_THREAD_COUNT) , cpus => Ok (unsafe { NonZero :: new_unchecked (cpus) }) , } }
}

macro_rules! current_os_id_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_os_id in module {}", module_path!());
    };
}

mkfn!{
    current_os_id_introspect!();
    pub fn current_os_id () -> Option < u64 > { let id : u32 = unsafe { c :: GetCurrentThreadId () } ; if id == 0 { None } else { Some (id . into ()) } }
}

macro_rules! set_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_name in module {}", module_path!());
    };
}

mkfn!{
    set_name_introspect!();
    pub fn set_name (name : & CStr) { if let Ok (utf8) = name . to_str () { if let Ok (utf16) = to_u16s (utf8) { unsafe { set_name_wide (& utf16) } } ; } ; }
}

macro_rules! set_name_wide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_name_wide in module {}", module_path!());
    };
}

mkfn!{
    set_name_wide_introspect!();
    # [doc = " # Safety"] # [doc = ""] # [doc = " `name` must end with a zero value"] pub unsafe fn set_name_wide (name : & [u16]) { unsafe { c :: SetThreadDescription (c :: GetCurrentThread () , name . as_ptr ()) } ; }
}

macro_rules! sleep_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sleep in module {}", module_path!());
    };
}

mkfn!{
    sleep_introspect!();
    pub fn sleep (dur : Duration) { fn high_precision_sleep (dur : Duration) -> Result < () , () > { let timer = WaitableTimer :: high_resolution () ? ; timer . set (dur) ? ; timer . wait () } if dur . is_zero () || high_precision_sleep (dur) . is_err () { unsafe { c :: Sleep (dur2timeout (dur)) } } }
}

macro_rules! yield_now_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function yield_now in module {}", module_path!());
    };
}

mkfn!{
    yield_now_introspect!();
    pub fn yield_now () { unsafe { c :: SwitchToThread () ; } }
}