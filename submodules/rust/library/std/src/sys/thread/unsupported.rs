mkuse!{use crate :: ffi :: CStr ;}
mkuse!{use crate :: io ;}
mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: time :: Duration ;}
mkitem!{mkstruct!{pub struct Thread (!) ;}}
mkitem!{pub const DEFAULT_MIN_STACK_SIZE : usize = 64 * 1024 ;}
mkitem!{mkimpl!{impl Thread { pub unsafe fn new (_stack : usize , _name : Option < & str > , _p : Box < dyn FnOnce () > ,) -> io :: Result < Thread > { Err (io :: Error :: UNSUPPORTED_PLATFORM) } pub fn join (self) { self . 0 } }}}

macro_rules! available_parallelism_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function available_parallelism in module {}", module_path!());
    };
}

mkfn!{
    available_parallelism_introspect!();
    pub fn available_parallelism () -> io :: Result < NonZero < usize > > { Err (io :: Error :: UNKNOWN_THREAD_COUNT) }
}

macro_rules! current_os_id_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_os_id in module {}", module_path!());
    };
}

mkfn!{
    current_os_id_introspect!();
    pub fn current_os_id () -> Option < u64 > { None }
}

macro_rules! yield_now_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function yield_now in module {}", module_path!());
    };
}

mkfn!{
    yield_now_introspect!();
    pub fn yield_now () { }
}

macro_rules! set_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_name in module {}", module_path!());
    };
}

mkfn!{
    set_name_introspect!();
    pub fn set_name (_name : & CStr) { }
}

macro_rules! sleep_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sleep in module {}", module_path!());
    };
}

mkfn!{
    sleep_introspect!();
    pub fn sleep (_dur : Duration) { panic ! ("can't sleep") ; }
}