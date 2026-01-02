mkuse!{use crate :: ffi :: { CStr , OsStr , OsString } ;}
mkuse!{use crate :: marker :: PhantomData ;}
mkuse!{use crate :: os :: wasi :: prelude :: * ;}
mkuse!{use crate :: path :: { self , PathBuf } ;}
mkuse!{use crate :: sys :: common :: small_c_string :: run_path_with_cstr ;}
mkuse!{use crate :: sys :: unsupported ;}
mkuse!{use crate :: { fmt , io , str } ;}
mkmod!{libc, { 
                getname!(libc);
                getsrc!(libc);
                getpath!(libc);
                get_deps!(libc);
                get_crates!(libc);
                mkinclude!(libc);
                mkuse!{pub use libc :: * ;}
mkitem!{unsafe extern "C" { pub fn getcwd (buf : * mut c_char , size : size_t) -> * mut c_char ; pub fn chdir (dir : * const c_char) -> c_int ; pub fn __wasilibc_get_environ () -> * mut * mut c_char ; }} 
            }}

macro_rules! errno_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function errno in module {}", module_path!());
    };
}

mkfn!{
    errno_introspect!();
    pub fn errno () -> i32 { unsafe extern "C" { # [thread_local] static errno : libc :: c_int ; } unsafe { errno as i32 } }
}

macro_rules! error_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function error_string in module {}", module_path!());
    };
}

mkfn!{
    error_string_introspect!();
    pub fn error_string (errno : i32) -> String { let mut buf = [0 as libc :: c_char ; 1024] ; let p = buf . as_mut_ptr () ; unsafe { if libc :: strerror_r (errno as libc :: c_int , p , buf . len ()) < 0 { panic ! ("strerror_r failure") ; } str :: from_utf8 (CStr :: from_ptr (p) . to_bytes ()) . unwrap () . to_owned () } }
}

macro_rules! getcwd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getcwd in module {}", module_path!());
    };
}

mkfn!{
    getcwd_introspect!();
    pub fn getcwd () -> io :: Result < PathBuf > { let mut buf = Vec :: with_capacity (512) ; loop { unsafe { let ptr = buf . as_mut_ptr () as * mut libc :: c_char ; if ! libc :: getcwd (ptr , buf . capacity ()) . is_null () { let len = CStr :: from_ptr (buf . as_ptr () as * const libc :: c_char) . to_bytes () . len () ; buf . set_len (len) ; buf . shrink_to_fit () ; return Ok (PathBuf :: from (OsString :: from_vec (buf))) ; } else { let error = io :: Error :: last_os_error () ; if error . raw_os_error () != Some (libc :: ERANGE) { return Err (error) ; } } let cap = buf . capacity () ; buf . set_len (cap) ; buf . reserve (1) ; } } }
}

macro_rules! chdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function chdir in module {}", module_path!());
    };
}

mkfn!{
    chdir_introspect!();
    pub fn chdir (p : & path :: Path) -> io :: Result < () > { let result = run_path_with_cstr (p , & | p | unsafe { Ok (libc :: chdir (p . as_ptr ())) }) ? ; match result == (0 as libc :: c_int) { true => Ok (()) , false => Err (io :: Error :: last_os_error ()) , } }
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
mkitem!{mkimpl!{impl fmt :: Display for JoinPathsError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "not supported on wasm yet" . fmt (f) } }}}
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

macro_rules! page_size_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function page_size in module {}", module_path!());
    };
}

mkfn!{
    page_size_introspect!();
    # [allow (dead_code)] pub fn page_size () -> usize { unsafe { libc :: sysconf (libc :: _SC_PAGESIZE) as usize } }
}

macro_rules! temp_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function temp_dir in module {}", module_path!());
    };
}

mkfn!{
    temp_dir_introspect!();
    pub fn temp_dir () -> PathBuf { panic ! ("no filesystem on wasm") }
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
    pub fn exit (code : i32) -> ! { unsafe { libc :: exit (code) } }
}

macro_rules! getpid_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getpid in module {}", module_path!());
    };
}

mkfn!{
    getpid_introspect!();
    pub fn getpid () -> u32 { panic ! ("unsupported") ; }
}
mkitem!{mktrait!{# [doc (hidden)] pub trait IsMinusOne { fn is_minus_one (& self) -> bool ; }}}
mkitem!{macro_rules ! impl_is_minus_one { ($ ($ t : ident) *) => ($ (impl IsMinusOne for $ t { fn is_minus_one (& self) -> bool { * self == - 1 } }) *) }}
mkitem!{impl_is_minus_one ! { i8 i16 i32 i64 isize }}

macro_rules! cvt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cvt in module {}", module_path!());
    };
}

mkfn!{
    cvt_introspect!();
    pub fn cvt < T : IsMinusOne > (t : T) -> io :: Result < T > { if t . is_minus_one () { Err (io :: Error :: last_os_error ()) } else { Ok (t) } }
}