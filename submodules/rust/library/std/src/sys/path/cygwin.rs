mkuse!{use crate :: ffi :: OsString ;}
mkuse!{use crate :: os :: unix :: ffi :: OsStringExt ;}
mkuse!{use crate :: path :: { Path , PathBuf } ;}
mkuse!{use crate :: sys :: common :: small_c_string :: run_path_with_cstr ;}
mkuse!{use crate :: sys :: cvt ;}
mkuse!{use crate :: { io , ptr } ;}

macro_rules! is_sep_byte_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_sep_byte in module {}", module_path!());
    };
}

mkfn!{
    is_sep_byte_introspect!();
    # [inline] pub fn is_sep_byte (b : u8) -> bool { b == b'/' || b == b'\\' }
}

macro_rules! is_verbatim_sep_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_verbatim_sep in module {}", module_path!());
    };
}

mkfn!{
    is_verbatim_sep_introspect!();
    # [doc = " Cygwin always prefers `/` over `\\`, and it always converts all `/` to `\\`"] # [doc = " internally when calling Win32 APIs. Therefore, the server component of path"] # [doc = " `\\\\?\\UNC\\localhost/share` is `localhost/share` on Win32, but `localhost`"] # [doc = " on Cygwin."] # [inline] pub fn is_verbatim_sep (b : u8) -> bool { b == b'/' || b == b'\\' }
}
mkuse!{pub use super :: windows_prefix :: parse_prefix ;}
mkitem!{pub const MAIN_SEP_STR : & str = "/" ;}
mkitem!{pub const MAIN_SEP : char = '/' ;}
mkitem!{unsafe extern "C" { fn cygwin_conv_path (what : libc :: c_uint , from : * const libc :: c_char , to : * mut u8 , size : libc :: size_t ,) -> libc :: ssize_t ; }}
mkitem!{const CCP_WIN_A_TO_POSIX : libc :: c_uint = 2 ;}

macro_rules! absolute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function absolute in module {}", module_path!());
    };
}

mkfn!{
    absolute_introspect!();
    # [doc = " Make a POSIX path absolute."] pub (crate) fn absolute (path : & Path) -> io :: Result < PathBuf > { run_path_with_cstr (path , & | path | { let conv = CCP_WIN_A_TO_POSIX ; let size = cvt (unsafe { cygwin_conv_path (conv , path . as_ptr () , ptr :: null_mut () , 0) }) ? ; debug_assert ! (size >= 1) ; let size = size as usize ; let mut buffer = Vec :: with_capacity (size) ; cvt (unsafe { cygwin_conv_path (conv , path . as_ptr () , buffer . as_mut_ptr () , size) }) ? ; unsafe { buffer . set_len (size - 1) ; } Ok (PathBuf :: from (OsString :: from_vec (buffer))) }) . map (| path | { if path . prefix () . is_some () { return path ; } let mut components = path . components () ; let path_os = path . as_os_str () . as_encoded_bytes () ; let mut normalized = if path_os . starts_with (b"//") && ! path_os . starts_with (b"///") { components . next () ; PathBuf :: from ("//") } else { PathBuf :: new () } ; normalized . extend (components) ; if path_os . ends_with (b"/") { normalized . push ("") ; } normalized }) }
}

macro_rules! is_absolute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_absolute in module {}", module_path!());
    };
}

mkfn!{
    is_absolute_introspect!();
    pub (crate) fn is_absolute (path : & Path) -> bool { if path . as_os_str () . as_encoded_bytes () . starts_with (b"\\") { path . has_root () && path . prefix () . is_some () } else { path . has_root () } }
}