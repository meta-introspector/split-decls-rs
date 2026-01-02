mkuse!{use super :: unsupported ;}
mkuse!{use crate :: ffi :: { OsStr , OsString } ;}
mkuse!{use crate :: marker :: PhantomData ;}
mkuse!{use crate :: os :: xous :: ffi :: Error as XousError ;}
mkuse!{use crate :: path :: { self , PathBuf } ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicPtr , Ordering } ;}
mkuse!{use crate :: { fmt , io } ;}
mkmod!{params, { 
                getname!(params);
                getsrc!(params);
                getpath!(params);
                get_deps!(params);
                get_crates!(params);
                mkinclude!(params);
                 
            }}
mkitem!{static PARAMS_ADDRESS : Atomic < * mut u8 > = AtomicPtr :: new (core :: ptr :: null_mut ()) ;}
mkmod!{eh_unwinding, { 
                getname!(eh_unwinding);
                getsrc!(eh_unwinding);
                getpath!(eh_unwinding);
                get_deps!(eh_unwinding);
                get_crates!(eh_unwinding);
                mkinclude!(eh_unwinding);
                mkitem!{mkstruct!{pub (crate) struct EhFrameFinder ;}}
mkitem!{pub (crate) static mut EH_FRAME_ADDRESS : usize = 0 ;}
mkitem!{pub (crate) static EH_FRAME_SETTINGS : EhFrameFinder = EhFrameFinder ;}
mkitem!{mkimpl!{unsafe impl unwind :: EhFrameFinder for EhFrameFinder { fn find (& self , _pc : usize) -> Option < unwind :: FrameInfo > { if unsafe { EH_FRAME_ADDRESS == 0 } { None } else { Some (unwind :: FrameInfo { text_base : None , kind : unwind :: FrameInfoKind :: EhFrame (unsafe { EH_FRAME_ADDRESS }) , }) } } }}} 
            }}
mkmod!{c_compat, { 
                getname!(c_compat);
                getsrc!(c_compat);
                getpath!(c_compat);
                get_deps!(c_compat);
                get_crates!(c_compat);
                mkinclude!(c_compat);
                mkuse!{use crate :: os :: xous :: ffi :: exit ;}
mkitem!{unsafe extern "C" { fn main () -> u32 ; }}

macro_rules! abort_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function abort in module {}", module_path!());
    };
}

mkfn!{
    abort_introspect!();
    # [unsafe (no_mangle)] pub extern "C" fn abort () { exit (1) ; }
}

macro_rules! _start_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _start in module {}", module_path!());
    };
}

mkfn!{
    _start_introspect!();
    # [unsafe (no_mangle)] pub extern "C" fn _start (eh_frame : usize , params_address : usize) { # [cfg (feature = "panic_unwind")] { unsafe { super :: eh_unwinding :: EH_FRAME_ADDRESS = eh_frame } ; unwind :: set_custom_eh_frame_finder (& super :: eh_unwinding :: EH_FRAME_SETTINGS) . ok () ; } if params_address != 0 { let params_address = crate :: ptr :: with_exposed_provenance_mut :: < u8 > (params_address) ; if unsafe { super :: params :: ApplicationParameters :: new_from_ptr (params_address) . is_some () } { super :: PARAMS_ADDRESS . store (params_address , core :: sync :: atomic :: Ordering :: Relaxed) ; } } exit (unsafe { main () }) ; }
} 
            }}

macro_rules! errno_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function errno in module {}", module_path!());
    };
}

mkfn!{
    errno_introspect!();
    pub fn errno () -> i32 { 0 }
}

macro_rules! error_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function error_string in module {}", module_path!());
    };
}

mkfn!{
    error_string_introspect!();
    pub fn error_string (errno : i32) -> String { Into :: < XousError > :: into (errno) . to_string () }
}

macro_rules! getcwd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getcwd in module {}", module_path!());
    };
}

mkfn!{
    getcwd_introspect!();
    pub fn getcwd () -> io :: Result < PathBuf > { unsupported () }
}

macro_rules! chdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function chdir in module {}", module_path!());
    };
}

mkfn!{
    chdir_introspect!();
    pub fn chdir (_ : & path :: Path) -> io :: Result < () > { unsupported () }
}
mkitem!{mkstruct!{pub struct SplitPaths < 'a > (! , PhantomData < & 'a () >) ;}}

macro_rules! split_paths_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function split_paths in module {}", module_path!());
    };
}

mkfn!{
    split_paths_introspect!();
    pub fn split_paths (_unparsed : & OsStr) -> SplitPaths < '_ > { panic ! ("unsupported") }
}
mkitem!{mkimpl!{impl < 'a > Iterator for SplitPaths < 'a > { type Item = PathBuf ; fn next (& mut self) -> Option < PathBuf > { self . 0 } }}}
mkitem!{mkstruct!{# [derive (Debug)] pub struct JoinPathsError ;}}

macro_rules! join_paths_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function join_paths in module {}", module_path!());
    };
}

mkfn!{
    join_paths_introspect!();
    pub fn join_paths < I , T > (_paths : I) -> Result < OsString , JoinPathsError > where I : Iterator < Item = T > , T : AsRef < OsStr > , { Err (JoinPathsError) }
}
mkitem!{mkimpl!{impl fmt :: Display for JoinPathsError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "not supported on this platform yet" . fmt (f) } }}}
mkitem!{mkimpl!{impl crate :: error :: Error for JoinPathsError { }}}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    pub fn current_exe () -> io :: Result < PathBuf > { unsupported () }
}

macro_rules! get_application_parameters_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_application_parameters in module {}", module_path!());
    };
}

mkfn!{
    get_application_parameters_introspect!();
    pub (crate) fn get_application_parameters () -> Option < params :: ApplicationParameters > { let params_address = PARAMS_ADDRESS . load (Ordering :: Relaxed) ; unsafe { params :: ApplicationParameters :: new_from_ptr (params_address) } }
}

macro_rules! temp_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function temp_dir in module {}", module_path!());
    };
}

mkfn!{
    temp_dir_introspect!();
    pub fn temp_dir () -> PathBuf { panic ! ("no filesystem on this platform") }
}

macro_rules! home_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function home_dir in module {}", module_path!());
    };
}

mkfn!{
    home_dir_introspect!();
    pub fn home_dir () -> Option < PathBuf > { None }
}

macro_rules! exit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exit in module {}", module_path!());
    };
}

mkfn!{
    exit_introspect!();
    pub fn exit (code : i32) -> ! { crate :: os :: xous :: ffi :: exit (code as u32) ; }
}

macro_rules! getpid_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getpid in module {}", module_path!());
    };
}

mkfn!{
    getpid_introspect!();
    pub fn getpid () -> u32 { panic ! ("no pids on this platform") }
}