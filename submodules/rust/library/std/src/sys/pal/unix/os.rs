mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{use libc :: { c_char , c_int , c_void } ;}
mkuse!{use crate :: ffi :: { CStr , OsStr , OsString } ;}
mkuse!{use crate :: os :: unix :: prelude :: * ;}
mkuse!{use crate :: path :: { self , PathBuf } ;}
mkuse!{use crate :: sys :: common :: small_c_string :: run_path_with_cstr ;}
mkuse!{use crate :: sys :: cvt ;}
mkuse!{use crate :: { fmt , io , iter , mem , ptr , slice , str } ;}
mkitem!{const TMPBUF_SZ : usize = 128 ;}
mkitem!{const PATH_SEPARATOR : u8 = if cfg ! (target_os = "redox") { b';' } else { b':' } ;}
mkitem!{unsafe extern "C" { # [cfg (not (any (target_os = "dragonfly" , target_os = "vxworks" , target_os = "rtems")))] # [cfg_attr (any (target_os = "linux" , target_os = "emscripten" , target_os = "fuchsia" , target_os = "l4re" , target_os = "hurd" ,) , link_name = "__errno_location")] # [cfg_attr (any (target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin" , target_os = "android" , target_os = "redox" , target_os = "nuttx" , target_env = "newlib") , link_name = "__errno")] # [cfg_attr (any (target_os = "solaris" , target_os = "illumos") , link_name = "___errno")] # [cfg_attr (target_os = "nto" , link_name = "__get_errno_ptr")] # [cfg_attr (any (target_os = "freebsd" , target_vendor = "apple") , link_name = "__error")] # [cfg_attr (target_os = "haiku" , link_name = "_errnop")] # [cfg_attr (target_os = "aix" , link_name = "_Errno")] # [unsafe (ffi_const)] pub safe fn errno_location () -> * mut c_int ; }}

macro_rules! errno_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function errno in module {}", module_path!());
    };
}

mkfn!{
    errno_introspect!();
    # [doc = " Returns the platform-specific value of errno"] # [cfg (not (any (target_os = "dragonfly" , target_os = "vxworks" , target_os = "rtems")))] # [inline] pub fn errno () -> i32 { unsafe { (* errno_location ()) as i32 } }
}

macro_rules! set_errno_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_errno in module {}", module_path!());
    };
}

mkfn!{
    set_errno_introspect!();
    # [doc = " Sets the platform-specific value of errno"] # [cfg (all (not (target_os = "dragonfly") , not (target_os = "vxworks") , not (target_os = "rtems")))] # [allow (dead_code)] # [inline] pub fn set_errno (e : i32) { unsafe { * errno_location () = e as c_int } }
}

macro_rules! errno_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function errno in module {}", module_path!());
    };
}

mkfn!{
    errno_introspect!();
    # [cfg (target_os = "vxworks")] # [inline] pub fn errno () -> i32 { unsafe { libc :: errnoGet () } }
}

macro_rules! errno_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function errno in module {}", module_path!());
    };
}

mkfn!{
    errno_introspect!();
    # [cfg (target_os = "rtems")] # [inline] pub fn errno () -> i32 { unsafe extern "C" { # [thread_local] static _tls_errno : c_int ; } unsafe { _tls_errno as i32 } }
}

macro_rules! errno_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function errno in module {}", module_path!());
    };
}

mkfn!{
    errno_introspect!();
    # [cfg (target_os = "dragonfly")] # [inline] pub fn errno () -> i32 { unsafe extern "C" { # [thread_local] static errno : c_int ; } unsafe { errno as i32 } }
}

macro_rules! set_errno_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_errno in module {}", module_path!());
    };
}

mkfn!{
    set_errno_introspect!();
    # [cfg (target_os = "dragonfly")] # [allow (dead_code)] # [inline] pub fn set_errno (e : i32) { unsafe extern "C" { # [thread_local] static mut errno : c_int ; } unsafe { errno = e ; } }
}

macro_rules! error_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function error_string in module {}", module_path!());
    };
}

mkfn!{
    error_string_introspect!();
    # [doc = " Gets a detailed string description for the given error number."] pub fn error_string (errno : i32) -> String { unsafe extern "C" { # [cfg_attr (all (any (target_os = "linux" , target_os = "hurd" , target_env = "newlib" , target_os = "cygwin") , not (target_env = "ohos")) , link_name = "__xpg_strerror_r")] fn strerror_r (errnum : c_int , buf : * mut c_char , buflen : libc :: size_t) -> c_int ; } let mut buf = [0 as c_char ; TMPBUF_SZ] ; let p = buf . as_mut_ptr () ; unsafe { if strerror_r (errno as c_int , p , buf . len ()) < 0 { panic ! ("strerror_r failure") ; } let p = p as * const _ ; String :: from_utf8_lossy (CStr :: from_ptr (p) . to_bytes ()) . into () } }
}

macro_rules! getcwd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getcwd in module {}", module_path!());
    };
}

mkfn!{
    getcwd_introspect!();
    # [cfg (target_os = "espidf")] pub fn getcwd () -> io :: Result < PathBuf > { Ok (PathBuf :: from ("/")) }
}

macro_rules! getcwd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getcwd in module {}", module_path!());
    };
}

mkfn!{
    getcwd_introspect!();
    # [cfg (not (target_os = "espidf"))] pub fn getcwd () -> io :: Result < PathBuf > { let mut buf = Vec :: with_capacity (512) ; loop { unsafe { let ptr = buf . as_mut_ptr () as * mut libc :: c_char ; if ! libc :: getcwd (ptr , buf . capacity ()) . is_null () { let len = CStr :: from_ptr (buf . as_ptr () as * const libc :: c_char) . to_bytes () . len () ; buf . set_len (len) ; buf . shrink_to_fit () ; return Ok (PathBuf :: from (OsString :: from_vec (buf))) ; } else { let error = io :: Error :: last_os_error () ; if error . raw_os_error () != Some (libc :: ERANGE) { return Err (error) ; } } let cap = buf . capacity () ; buf . set_len (cap) ; buf . reserve (1) ; } } }
}

macro_rules! chdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function chdir in module {}", module_path!());
    };
}

mkfn!{
    chdir_introspect!();
    # [cfg (target_os = "espidf")] pub fn chdir (_p : & path :: Path) -> io :: Result < () > { super :: unsupported :: unsupported () }
}

macro_rules! chdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function chdir in module {}", module_path!());
    };
}

mkfn!{
    chdir_introspect!();
    # [cfg (not (target_os = "espidf"))] pub fn chdir (p : & path :: Path) -> io :: Result < () > { let result = run_path_with_cstr (p , & | p | unsafe { Ok (libc :: chdir (p . as_ptr ())) }) ? ; if result == 0 { Ok (()) } else { Err (io :: Error :: last_os_error ()) } }
}
mkitem!{pub type SplitPaths < 'a > = iter :: Map < slice :: Split < 'a , u8 , impl FnMut (& u8) -> bool + 'static > , impl FnMut (& [u8]) -> PathBuf + 'static , > ;}

macro_rules! split_paths_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function split_paths in module {}", module_path!());
    };
}

mkfn!{
    split_paths_introspect!();
    # [define_opaque (SplitPaths)] pub fn split_paths (unparsed : & OsStr) -> SplitPaths < '_ > { fn is_separator (& b : & u8) -> bool { b == PATH_SEPARATOR } fn into_pathbuf (part : & [u8]) -> PathBuf { PathBuf :: from (OsStr :: from_bytes (part)) } unparsed . as_bytes () . split (is_separator) . map (into_pathbuf) }
}
mkitem!{mkstruct!{# [derive (Debug)] pub struct JoinPathsError ;}}

macro_rules! join_paths_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function join_paths in module {}", module_path!());
    };
}

mkfn!{
    join_paths_introspect!();
    pub fn join_paths < I , T > (paths : I) -> Result < OsString , JoinPathsError > where I : Iterator < Item = T > , T : AsRef < OsStr > , { let mut joined = Vec :: new () ; for (i , path) in paths . enumerate () { let path = path . as_ref () . as_bytes () ; if i > 0 { joined . push (PATH_SEPARATOR) } if path . contains (& PATH_SEPARATOR) { return Err (JoinPathsError) ; } joined . extend_from_slice (path) ; } Ok (OsStringExt :: from_vec (joined)) }
}
mkitem!{mkimpl!{impl fmt :: Display for JoinPathsError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "path segment contains separator `{}`" , char :: from (PATH_SEPARATOR)) } }}}
mkitem!{mkimpl!{impl crate :: error :: Error for JoinPathsError { }}}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    # [cfg (target_os = "aix")] pub fn current_exe () -> io :: Result < PathBuf > { # [cfg (test)] use realstd :: env ; # [cfg (not (test))] use crate :: env ; use crate :: io :: ErrorKind ; let exe_path = env :: args () . next () . ok_or (io :: const_error ! (ErrorKind :: NotFound , "an executable path was not found because no arguments were provided through argv" ,)) ? ; let path = PathBuf :: from (exe_path) ; if path . is_absolute () { return path . canonicalize () ; } if let Some (pstr) = path . to_str () && pstr . contains ("/") { return getcwd () . map (| cwd | cwd . join (path)) ? . canonicalize () ; } if let Some (p) = env :: var_os (OsStr :: from_bytes ("PATH" . as_bytes ())) { for search_path in split_paths (& p) { let pb = search_path . join (& path) ; if pb . is_file () && let Ok (metadata) = crate :: fs :: metadata (& pb) && metadata . permissions () . mode () & 0o111 != 0 { return pb . canonicalize () ; } } } Err (io :: const_error ! (ErrorKind :: NotFound , "an executable path was not found")) }
}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    # [cfg (any (target_os = "freebsd" , target_os = "dragonfly"))] pub fn current_exe () -> io :: Result < PathBuf > { unsafe { let mut mib = [libc :: CTL_KERN as c_int , libc :: KERN_PROC as c_int , libc :: KERN_PROC_PATHNAME as c_int , - 1 as c_int ,] ; let mut sz = 0 ; cvt (libc :: sysctl (mib . as_mut_ptr () , mib . len () as libc :: c_uint , ptr :: null_mut () , & mut sz , ptr :: null_mut () , 0 ,)) ? ; if sz == 0 { return Err (io :: Error :: last_os_error ()) ; } let mut v : Vec < u8 > = Vec :: with_capacity (sz) ; cvt (libc :: sysctl (mib . as_mut_ptr () , mib . len () as libc :: c_uint , v . as_mut_ptr () as * mut libc :: c_void , & mut sz , ptr :: null_mut () , 0 ,)) ? ; if sz == 0 { return Err (io :: Error :: last_os_error ()) ; } v . set_len (sz - 1) ; Ok (PathBuf :: from (OsString :: from_vec (v))) } }
}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    # [cfg (target_os = "netbsd")] pub fn current_exe () -> io :: Result < PathBuf > { fn sysctl () -> io :: Result < PathBuf > { unsafe { let mib = [libc :: CTL_KERN , libc :: KERN_PROC_ARGS , - 1 , libc :: KERN_PROC_PATHNAME] ; let mut path_len : usize = 0 ; cvt (libc :: sysctl (mib . as_ptr () , mib . len () as libc :: c_uint , ptr :: null_mut () , & mut path_len , ptr :: null () , 0 ,)) ? ; if path_len <= 1 { return Err (io :: const_error ! (io :: ErrorKind :: Uncategorized , "KERN_PROC_PATHNAME sysctl returned zero-length string" ,)) ; } let mut path : Vec < u8 > = Vec :: with_capacity (path_len) ; cvt (libc :: sysctl (mib . as_ptr () , mib . len () as libc :: c_uint , path . as_ptr () as * mut libc :: c_void , & mut path_len , ptr :: null () , 0 ,)) ? ; path . set_len (path_len - 1) ; Ok (PathBuf :: from (OsString :: from_vec (path))) } } fn procfs () -> io :: Result < PathBuf > { let curproc_exe = path :: Path :: new ("/proc/curproc/exe") ; if curproc_exe . is_file () { return crate :: fs :: read_link (curproc_exe) ; } Err (io :: const_error ! (io :: ErrorKind :: Uncategorized , "/proc/curproc/exe doesn't point to regular file." ,)) } sysctl () . or_else (| _ | procfs ()) }
}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    # [cfg (target_os = "openbsd")] pub fn current_exe () -> io :: Result < PathBuf > { unsafe { let mut mib = [libc :: CTL_KERN , libc :: KERN_PROC_ARGS , libc :: getpid () , libc :: KERN_PROC_ARGV] ; let mib = mib . as_mut_ptr () ; let mut argv_len = 0 ; cvt (libc :: sysctl (mib , 4 , ptr :: null_mut () , & mut argv_len , ptr :: null_mut () , 0)) ? ; let mut argv = Vec :: < * const libc :: c_char > :: with_capacity (argv_len as usize) ; cvt (libc :: sysctl (mib , 4 , argv . as_mut_ptr () as * mut _ , & mut argv_len , ptr :: null_mut () , 0)) ? ; argv . set_len (argv_len as usize) ; if argv [0] . is_null () { return Err (io :: const_error ! (io :: ErrorKind :: Uncategorized , "no current exe available")) ; } let argv0 = CStr :: from_ptr (argv [0]) . to_bytes () ; if argv0 [0] == b'.' || argv0 . iter () . any (| b | * b == b'/') { crate :: fs :: canonicalize (OsStr :: from_bytes (argv0)) } else { Ok (PathBuf :: from (OsStr :: from_bytes (argv0))) } } }
}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    # [cfg (any (target_os = "linux" , target_os = "cygwin" , target_os = "hurd" , target_os = "android" , target_os = "nuttx" , target_os = "emscripten"))] pub fn current_exe () -> io :: Result < PathBuf > { match crate :: fs :: read_link ("/proc/self/exe") { Err (ref e) if e . kind () == io :: ErrorKind :: NotFound => Err (io :: const_error ! (io :: ErrorKind :: Uncategorized , "no /proc/self/exe available. Is /proc mounted?" ,)) , other => other , } }
}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    # [cfg (target_os = "nto")] pub fn current_exe () -> io :: Result < PathBuf > { let mut e = crate :: fs :: read ("/proc/self/exefile") ? ; if let Some (0) = e . last () { e . pop () ; } Ok (PathBuf :: from (OsString :: from_vec (e))) }
}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    # [cfg (target_vendor = "apple")] pub fn current_exe () -> io :: Result < PathBuf > { unsafe { let mut sz : u32 = 0 ; # [expect (deprecated)] libc :: _NSGetExecutablePath (ptr :: null_mut () , & mut sz) ; if sz == 0 { return Err (io :: Error :: last_os_error ()) ; } let mut v : Vec < u8 > = Vec :: with_capacity (sz as usize) ; # [expect (deprecated)] let err = libc :: _NSGetExecutablePath (v . as_mut_ptr () as * mut i8 , & mut sz) ; if err != 0 { return Err (io :: Error :: last_os_error ()) ; } v . set_len (sz as usize - 1) ; Ok (PathBuf :: from (OsString :: from_vec (v))) } }
}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    # [cfg (any (target_os = "solaris" , target_os = "illumos"))] pub fn current_exe () -> io :: Result < PathBuf > { if let Ok (path) = crate :: fs :: read_link ("/proc/self/path/a.out") { Ok (path) } else { unsafe { let path = libc :: getexecname () ; if path . is_null () { Err (io :: Error :: last_os_error ()) } else { let filename = CStr :: from_ptr (path) . to_bytes () ; let path = PathBuf :: from (< OsStr as OsStrExt > :: from_bytes (filename)) ; if filename [0] == b'/' { Ok (path) } else { getcwd () . map (| cwd | cwd . join (path)) } } } } }
}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    # [cfg (target_os = "haiku")] pub fn current_exe () -> io :: Result < PathBuf > { let mut name = vec ! [0 ; libc :: PATH_MAX as usize] ; unsafe { let result = libc :: find_path (crate :: ptr :: null_mut () , libc :: B_FIND_PATH_IMAGE_PATH , crate :: ptr :: null_mut () , name . as_mut_ptr () , name . len () ,) ; if result != libc :: B_OK { use crate :: io :: ErrorKind ; Err (io :: const_error ! (ErrorKind :: Uncategorized , "error getting executable path")) } else { let name = CStr :: from_ptr (name . as_ptr ()) . to_bytes () ; Ok (PathBuf :: from (OsStr :: from_bytes (name))) } } }
}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    # [cfg (target_os = "redox")] pub fn current_exe () -> io :: Result < PathBuf > { crate :: fs :: read_to_string ("/scheme/sys/exe") . map (PathBuf :: from) }
}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    # [cfg (target_os = "rtems")] pub fn current_exe () -> io :: Result < PathBuf > { crate :: fs :: read_to_string ("sys:exe") . map (PathBuf :: from) }
}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    # [cfg (target_os = "l4re")] pub fn current_exe () -> io :: Result < PathBuf > { use crate :: io :: ErrorKind ; Err (io :: const_error ! (ErrorKind :: Unsupported , "not yet implemented!")) }
}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    # [cfg (target_os = "vxworks")] pub fn current_exe () -> io :: Result < PathBuf > { # [cfg (test)] use realstd :: env ; # [cfg (not (test))] use crate :: env ; let exe_path = env :: args () . next () . unwrap () ; let path = path :: Path :: new (& exe_path) ; path . canonicalize () }
}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    # [cfg (any (target_os = "espidf" , target_os = "horizon" , target_os = "vita"))] pub fn current_exe () -> io :: Result < PathBuf > { super :: unsupported :: unsupported () }
}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    # [cfg (target_os = "fuchsia")] pub fn current_exe () -> io :: Result < PathBuf > { # [cfg (test)] use realstd :: env ; # [cfg (not (test))] use crate :: env ; use crate :: io :: ErrorKind ; let exe_path = env :: args () . next () . ok_or (io :: const_error ! (ErrorKind :: Uncategorized , "an executable path was not found because no arguments were provided through argv" ,)) ? ; let path = PathBuf :: from (exe_path) ; if ! path . is_absolute () { getcwd () . map (| cwd | cwd . join (path)) } else { Ok (path) } }
}

macro_rules! page_size_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function page_size in module {}", module_path!());
    };
}

mkfn!{
    page_size_introspect!();
    # [cfg (not (target_os = "espidf"))] pub fn page_size () -> usize { unsafe { libc :: sysconf (libc :: _SC_PAGESIZE) as usize } }
}

macro_rules! confstr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function confstr in module {}", module_path!());
    };
}

mkfn!{
    confstr_introspect!();
    # [cfg (all (target_vendor = "apple" , not (miri)))] fn confstr (key : c_int , size_hint : Option < usize >) -> io :: Result < OsString > { let mut buf : Vec < u8 > = Vec :: with_capacity (0) ; let mut bytes_needed_including_nul = size_hint . unwrap_or_else (| | { unsafe { libc :: confstr (key , core :: ptr :: null_mut () , 0) } }) . max (1) ; while bytes_needed_including_nul > buf . capacity () { buf . reserve (bytes_needed_including_nul) ; bytes_needed_including_nul = unsafe { libc :: confstr (key , buf . as_mut_ptr () . cast :: < c_char > () , buf . capacity ()) } ; } if bytes_needed_including_nul == 0 { return Err (io :: Error :: last_os_error ()) ; } unsafe { buf . set_len (bytes_needed_including_nul) ; let last_byte = buf . pop () ; assert_eq ! (last_byte , Some (0) , "`confstr` provided a string which wasn't nul-terminated") ; } ; Ok (OsString :: from_vec (buf)) }
}

macro_rules! darwin_temp_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function darwin_temp_dir in module {}", module_path!());
    };
}

mkfn!{
    darwin_temp_dir_introspect!();
    # [cfg (all (target_vendor = "apple" , not (miri)))] fn darwin_temp_dir () -> PathBuf { confstr (libc :: _CS_DARWIN_USER_TEMP_DIR , Some (64)) . map (PathBuf :: from) . unwrap_or_else (| _ | { PathBuf :: from ("/tmp") }) }
}

macro_rules! temp_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function temp_dir in module {}", module_path!());
    };
}

mkfn!{
    temp_dir_introspect!();
    pub fn temp_dir () -> PathBuf { crate :: env :: var_os ("TMPDIR") . map (PathBuf :: from) . unwrap_or_else (| | { cfg_select ! { all (target_vendor = "apple" , not (miri)) => darwin_temp_dir () , target_os = "android" => PathBuf :: from ("/data/local/tmp") , _ => PathBuf :: from ("/tmp") , } }) }
}

macro_rules! home_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function home_dir in module {}", module_path!());
    };
}

mkfn!{
    home_dir_introspect!();
    pub fn home_dir () -> Option < PathBuf > { return crate :: env :: var_os ("HOME") . filter (| s | ! s . is_empty ()) . or_else (| | unsafe { fallback () }) . map (PathBuf :: from) ; # [cfg (any (target_os = "android" , target_os = "emscripten" , target_os = "redox" , target_os = "vxworks" , target_os = "espidf" , target_os = "horizon" , target_os = "vita" , target_os = "nuttx" , all (target_vendor = "apple" , not (target_os = "macos")) ,))] unsafe fn fallback () -> Option < OsString > { None } # [cfg (not (any (target_os = "android" , target_os = "emscripten" , target_os = "redox" , target_os = "vxworks" , target_os = "espidf" , target_os = "horizon" , target_os = "vita" , target_os = "nuttx" , all (target_vendor = "apple" , not (target_os = "macos")) ,)))] unsafe fn fallback () -> Option < OsString > { let amt = match libc :: sysconf (libc :: _SC_GETPW_R_SIZE_MAX) { n if n < 0 => 512 as usize , n => n as usize , } ; let mut buf = Vec :: with_capacity (amt) ; let mut p = mem :: MaybeUninit :: < libc :: passwd > :: uninit () ; let mut result = ptr :: null_mut () ; match libc :: getpwuid_r (libc :: getuid () , p . as_mut_ptr () , buf . as_mut_ptr () , buf . capacity () , & mut result ,) { 0 if ! result . is_null () => { let ptr = (* result) . pw_dir as * const _ ; let bytes = CStr :: from_ptr (ptr) . to_bytes () . to_vec () ; Some (OsStringExt :: from_vec (bytes)) } _ => None , } } }
}

macro_rules! exit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exit in module {}", module_path!());
    };
}

mkfn!{
    exit_introspect!();
    pub fn exit (code : i32) -> ! { crate :: sys :: exit_guard :: unique_thread_exit () ; unsafe { libc :: exit (code as c_int) } }
}

macro_rules! getpid_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getpid in module {}", module_path!());
    };
}

mkfn!{
    getpid_introspect!();
    pub fn getpid () -> u32 { unsafe { libc :: getpid () as u32 } }
}

macro_rules! getppid_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getppid in module {}", module_path!());
    };
}

mkfn!{
    getppid_introspect!();
    pub fn getppid () -> u32 { unsafe { libc :: getppid () as u32 } }
}

macro_rules! glibc_version_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function glibc_version in module {}", module_path!());
    };
}

mkfn!{
    glibc_version_introspect!();
    # [cfg (all (target_os = "linux" , target_env = "gnu"))] pub fn glibc_version () -> Option < (usize , usize) > { unsafe extern "C" { fn gnu_get_libc_version () -> * const libc :: c_char ; } let version_cstr = unsafe { CStr :: from_ptr (gnu_get_libc_version ()) } ; if let Ok (version_str) = version_cstr . to_str () { parse_glibc_version (version_str) } else { None } }
}

macro_rules! parse_glibc_version_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_glibc_version in module {}", module_path!());
    };
}

mkfn!{
    parse_glibc_version_introspect!();
    # [cfg (all (target_os = "linux" , target_env = "gnu"))] fn parse_glibc_version (version : & str) -> Option < (usize , usize) > { let mut parsed_ints = version . split ('.') . map (str :: parse :: < usize >) . fuse () ; match (parsed_ints . next () , parsed_ints . next ()) { (Some (Ok (major)) , Some (Ok (minor))) => Some ((major , minor)) , _ => None , } }
}