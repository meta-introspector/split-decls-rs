mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: { env , fs } ;}
mkuse!{use rustc_fs_util :: try_canonicalize ;}
mkuse!{use rustc_target :: spec :: Target ;}
mkuse!{use crate :: search_paths :: { PathKind , SearchPath } ;}
mkitem!{mkstruct!{# [derive (Clone)] pub struct FileSearch { cli_search_paths : Vec < SearchPath > , tlib_path : SearchPath , }}}
mkitem!{mkimpl!{impl FileSearch { pub fn cli_search_paths < 'b > (& 'b self , kind : PathKind) -> impl Iterator < Item = & 'b SearchPath > { self . cli_search_paths . iter () . filter (move | sp | sp . kind . matches (kind)) } pub fn search_paths < 'b > (& 'b self , kind : PathKind) -> impl Iterator < Item = & 'b SearchPath > { self . cli_search_paths . iter () . filter (move | sp | sp . kind . matches (kind)) . chain (std :: iter :: once (& self . tlib_path)) } pub fn new (cli_search_paths : & [SearchPath] , tlib_path : & SearchPath , target : & Target) -> Self { let this = FileSearch { cli_search_paths : cli_search_paths . to_owned () , tlib_path : tlib_path . clone () , } ; this . refine (& ["lib" , & target . staticlib_prefix , & target . dll_prefix]) } fn refine (mut self , allowed_prefixes : & [& str]) -> FileSearch { self . cli_search_paths . iter_mut () . for_each (| search_paths | search_paths . files . retain (allowed_prefixes)) ; self . tlib_path . files . retain (allowed_prefixes) ; self } }}}

macro_rules! make_target_lib_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_target_lib_path in module {}", module_path!());
    };
}

mkfn!{
    make_target_lib_path_introspect!();
    pub fn make_target_lib_path (sysroot : & Path , target_triple : & str) -> PathBuf { let rustlib_path = rustc_target :: relative_target_rustlib_path (sysroot , target_triple) ; sysroot . join (rustlib_path) . join ("lib") }
}

macro_rules! make_target_bin_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_target_bin_path in module {}", module_path!());
    };
}

mkfn!{
    make_target_bin_path_introspect!();
    # [doc = " Returns a path to the target's `bin` folder within its `rustlib` path in the sysroot. This is"] # [doc = " where binaries are usually installed, e.g. the self-contained linkers, lld-wrappers, LLVM tools,"] # [doc = " etc."] pub fn make_target_bin_path (sysroot : & Path , target_triple : & str) -> PathBuf { let rustlib_path = rustc_target :: relative_target_rustlib_path (sysroot , target_triple) ; sysroot . join (rustlib_path) . join ("bin") }
}

macro_rules! current_dll_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_dll_path in module {}", module_path!());
    };
}

mkfn!{
    current_dll_path_introspect!();
    # [cfg (unix)] fn current_dll_path () -> Result < PathBuf , String > { use std :: sync :: OnceLock ; static CURRENT_DLL_PATH : OnceLock < Result < PathBuf , String > > = OnceLock :: new () ; CURRENT_DLL_PATH . get_or_init (| | { use std :: ffi :: { CStr , OsStr } ; use std :: os :: unix :: prelude :: * ; # [cfg (not (target_os = "aix"))] unsafe { let addr = current_dll_path as usize as * mut _ ; let mut info = std :: mem :: zeroed () ; if libc :: dladdr (addr , & mut info) == 0 { return Err ("dladdr failed" . into ()) ; } # [cfg (target_os = "cygwin")] let fname_ptr = info . dli_fname . as_ptr () ; # [cfg (not (target_os = "cygwin"))] let fname_ptr = { assert ! (! info . dli_fname . is_null () , "dli_fname cannot be null") ; info . dli_fname } ; let bytes = CStr :: from_ptr (fname_ptr) . to_bytes () ; let os = OsStr :: from_bytes (bytes) ; try_canonicalize (Path :: new (os)) . map_err (| e | e . to_string ()) } # [cfg (target_os = "aix")] unsafe { let addr = current_dll_path as u64 ; let mut buffer = vec ! [std :: mem :: zeroed ::< libc :: ld_info > () ; 64] ; loop { if libc :: loadquery (libc :: L_GETINFO , buffer . as_mut_ptr () as * mut u8 , (size_of :: < libc :: ld_info > () * buffer . len ()) as u32 ,) >= 0 { break ; } else { if std :: io :: Error :: last_os_error () . raw_os_error () . unwrap () != libc :: ENOMEM { return Err ("loadquery failed" . into ()) ; } buffer . resize (buffer . len () * 2 , std :: mem :: zeroed :: < libc :: ld_info > ()) ; } } let mut current = buffer . as_mut_ptr () as * mut libc :: ld_info ; loop { let data_base = (* current) . ldinfo_dataorg as u64 ; let data_end = data_base + (* current) . ldinfo_datasize ; if (data_base .. data_end) . contains (& addr) { let bytes = CStr :: from_ptr (& (* current) . ldinfo_filename [0]) . to_bytes () ; let os = OsStr :: from_bytes (bytes) ; return try_canonicalize (Path :: new (os)) . map_err (| e | e . to_string ()) ; } if (* current) . ldinfo_next == 0 { break ; } current = (current as * mut i8) . offset ((* current) . ldinfo_next as isize) as * mut libc :: ld_info ; } return Err (format ! ("current dll's address {} is not in the load map" , addr)) ; } }) . clone () }
}

macro_rules! current_dll_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_dll_path in module {}", module_path!());
    };
}

mkfn!{
    current_dll_path_introspect!();
    # [cfg (windows)] fn current_dll_path () -> Result < PathBuf , String > { use std :: ffi :: OsString ; use std :: io ; use std :: os :: windows :: prelude :: * ; use windows :: Win32 :: Foundation :: HMODULE ; use windows :: Win32 :: System :: LibraryLoader :: { GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS , GetModuleFileNameW , GetModuleHandleExW , } ; use windows :: core :: PCWSTR ; let mut module = HMODULE :: default () ; unsafe { GetModuleHandleExW (GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS , PCWSTR (current_dll_path as * mut u16) , & mut module ,) } . map_err (| e | e . to_string ()) ? ; let mut filename = vec ! [0 ; 1024] ; let n = unsafe { GetModuleFileNameW (Some (module) , & mut filename) } as usize ; if n == 0 { return Err (format ! ("GetModuleFileNameW failed: {}" , io :: Error :: last_os_error ())) ; } if n >= filename . capacity () { return Err (format ! ("our buffer was too small? {}" , io :: Error :: last_os_error ())) ; } filename . truncate (n) ; let path = try_canonicalize (OsString :: from_wide (& filename)) . map_err (| e | e . to_string ()) ? ; Ok (rustc_fs_util :: fix_windows_verbatim_for_gcc (& path)) }
}

macro_rules! current_dll_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_dll_path in module {}", module_path!());
    };
}

mkfn!{
    current_dll_path_introspect!();
    # [cfg (target_os = "wasi")] fn current_dll_path () -> Result < PathBuf , String > { Err ("current_dll_path is not supported on WASI" . to_string ()) }
}

macro_rules! default_sysroot_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function default_sysroot in module {}", module_path!());
    };
}

mkfn!{
    default_sysroot_introspect!();
    # [doc = " This function checks if sysroot is found using env::args().next(), and if it"] # [doc = " is not found, finds sysroot from current rustc_driver dll."] pub (crate) fn default_sysroot () -> PathBuf { fn default_from_rustc_driver_dll () -> Result < PathBuf , String > { let dll = current_dll_path () ? ; let dir = dll . parent () . and_then (| p | p . parent ()) . ok_or_else (| | { format ! ("Could not move 2 levels upper using `parent()` on {}" , dll . display ()) }) ? ; let mut sysroot_dir = if dir . ends_with (crate :: config :: host_tuple ()) { dir . parent () . and_then (| p | p . parent ()) . and_then (| p | p . parent ()) . map (| s | s . to_owned ()) . ok_or_else (| | { format ! ("Could not move 3 levels upper using `parent()` on {}" , dir . display ()) }) ? } else { dir . to_owned () } ; if sysroot_dir . ends_with ("lib") { sysroot_dir = sysroot_dir . parent () . map (| real_sysroot | real_sysroot . to_owned ()) . ok_or_else (| | format ! ("Could not move to parent path of {}" , sysroot_dir . display ()) ,) ? } Ok (sysroot_dir) } fn from_env_args_next () -> Option < PathBuf > { let mut p = PathBuf :: from (env :: args_os () . next () ?) ; if fs :: read_link (& p) . is_err () { return None ; } p . pop () ; p . pop () ; let mut rustlib_path = rustc_target :: relative_target_rustlib_path (& p , "dummy") ; rustlib_path . pop () ; rustlib_path . exists () . then_some (p) } from_env_args_next () . unwrap_or_else (| | default_from_rustc_driver_dll () . expect ("Failed finding sysroot")) }
}