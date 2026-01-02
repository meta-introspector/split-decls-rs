mkuse!{use crate :: ffi :: { OsStr , OsString } ;}
mkuse!{use crate :: path :: { Path , PathBuf } ;}
mkuse!{use crate :: sys :: api :: utf16 ;}
mkuse!{use crate :: sys :: pal :: { c , fill_utf16_buf , os2path , to_u16s } ;}
mkuse!{use crate :: { io , ptr } ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{pub use super :: windows_prefix :: parse_prefix ;}
mkitem!{pub const MAIN_SEP_STR : & str = "\\" ;}
mkitem!{pub const MAIN_SEP : char = '\\' ;}
mkitem!{mkstruct!{# [doc = " A null terminated wide string."] # [repr (transparent)] pub struct WCStr ([u16]) ;}}
mkitem!{mkimpl!{impl WCStr { # [doc = " Convert a slice to a WCStr without checks."] # [doc = ""] # [doc = " Though it is memory safe, the slice should also not contain interior nulls"] # [doc = " as this may lead to unwanted truncation."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The slice must end in a null."] pub unsafe fn from_wchars_with_null_unchecked (s : & [u16]) -> & Self { unsafe { & * (s as * const [u16] as * const Self) } } pub fn as_ptr (& self) -> * const u16 { self . 0 . as_ptr () } pub fn count_bytes (& self) -> usize { self . 0 . len () } }}}

macro_rules! with_native_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function with_native_path in module {}", module_path!());
    };
}

mkfn!{
    with_native_path_introspect!();
    # [inline] pub fn with_native_path < T > (path : & Path , f : & dyn Fn (& WCStr) -> io :: Result < T >) -> io :: Result < T > { let path = maybe_verbatim (path) ? ; let path = unsafe { WCStr :: from_wchars_with_null_unchecked (& path) } ; f (path) }
}

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
    # [inline] pub fn is_verbatim_sep (b : u8) -> bool { b == b'\\' }
}

macro_rules! is_verbatim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_verbatim in module {}", module_path!());
    };
}

mkfn!{
    is_verbatim_introspect!();
    pub fn is_verbatim (path : & [u16]) -> bool { path . starts_with (utf16 ! (r"\\?\")) || path . starts_with (utf16 ! (r"\??\")) }
}

macro_rules! is_file_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_file_name in module {}", module_path!());
    };
}

mkfn!{
    is_file_name_introspect!();
    # [doc = " Returns true if `path` looks like a lone filename."] pub (crate) fn is_file_name (path : & OsStr) -> bool { ! path . as_encoded_bytes () . iter () . copied () . any (is_sep_byte) }
}

macro_rules! has_trailing_slash_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_trailing_slash in module {}", module_path!());
    };
}

mkfn!{
    has_trailing_slash_introspect!();
    pub (crate) fn has_trailing_slash (path : & OsStr) -> bool { let is_verbatim = path . as_encoded_bytes () . starts_with (br"\\?\") ; let is_separator = if is_verbatim { is_verbatim_sep } else { is_sep_byte } ; if let Some (& c) = path . as_encoded_bytes () . last () { is_separator (c) } else { false } }
}

macro_rules! append_suffix_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function append_suffix in module {}", module_path!());
    };
}

mkfn!{
    append_suffix_introspect!();
    # [doc = " Appends a suffix to a path."] # [doc = ""] # [doc = " Can be used to append an extension without removing an existing extension."] pub (crate) fn append_suffix (path : PathBuf , suffix : & OsStr) -> PathBuf { let mut path = OsString :: from (path) ; path . push (suffix) ; path . into () }
}

macro_rules! maybe_verbatim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function maybe_verbatim in module {}", module_path!());
    };
}

mkfn!{
    maybe_verbatim_introspect!();
    # [doc = " Returns a UTF-16 encoded path capable of bypassing the legacy `MAX_PATH` limits."] # [doc = ""] # [doc = " This path may or may not have a verbatim prefix."] pub (crate) fn maybe_verbatim (path : & Path) -> io :: Result < Vec < u16 > > { let path = to_u16s (path) ? ; get_long_path (path , true) }
}

macro_rules! get_long_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_long_path in module {}", module_path!());
    };
}

mkfn!{
    get_long_path_introspect!();
    # [doc = " Gets a normalized absolute path that can bypass path length limits."] # [doc = ""] # [doc = " Setting prefer_verbatim to true suggests a stronger preference for verbatim"] # [doc = " paths even when not strictly necessary. This allows the Windows API to avoid"] # [doc = " repeating our work. However, if the path may be given back to users or"] # [doc = " passed to other application then it's preferable to use non-verbatim paths"] # [doc = " when possible. Non-verbatim paths are better understood by users and handled"] # [doc = " by more software."] pub (crate) fn get_long_path (mut path : Vec < u16 > , prefer_verbatim : bool) -> io :: Result < Vec < u16 > > { const LEGACY_MAX_PATH : usize = 248 ; const SEP : u16 = b'\\' as _ ; const ALT_SEP : u16 = b'/' as _ ; const QUERY : u16 = b'?' as _ ; const COLON : u16 = b':' as _ ; const DOT : u16 = b'.' as _ ; const U : u16 = b'U' as _ ; const N : u16 = b'N' as _ ; const C : u16 = b'C' as _ ; const VERBATIM_PREFIX : & [u16] = & [SEP , SEP , QUERY , SEP] ; const NT_PREFIX : & [u16] = & [SEP , QUERY , QUERY , SEP] ; const UNC_PREFIX : & [u16] = & [SEP , SEP , QUERY , SEP , U , N , C , SEP] ; if path . starts_with (VERBATIM_PREFIX) || path . starts_with (NT_PREFIX) || path == [0] { return Ok (path) ; } else if path . len () < LEGACY_MAX_PATH { match path . as_slice () { [drive , COLON , 0] | [drive , COLON , SEP | ALT_SEP , ..] if * drive != SEP && * drive != ALT_SEP => { return Ok (path) ; } [SEP | ALT_SEP , SEP | ALT_SEP , ..] => return Ok (path) , _ => { } } } let lpfilename = path . as_ptr () ; fill_utf16_buf (| buffer , size | unsafe { c :: GetFullPathNameW (lpfilename , size , buffer , ptr :: null_mut ()) } , | mut absolute | { path . clear () ; if prefer_verbatim || absolute . len () + 1 >= LEGACY_MAX_PATH { let prefix = match absolute { [_ , COLON , SEP , ..] => VERBATIM_PREFIX , [SEP , SEP , DOT , SEP , ..] => { absolute = & absolute [4 ..] ; VERBATIM_PREFIX } [SEP , SEP , QUERY , SEP , ..] | [SEP , QUERY , QUERY , SEP , ..] => & [] , [SEP , SEP , ..] => { absolute = & absolute [2 ..] ; UNC_PREFIX } _ => & [] , } ; path . reserve_exact (prefix . len () + absolute . len () + 1) ; path . extend_from_slice (prefix) ; } else { path . reserve_exact (absolute . len () + 1) ; } path . extend_from_slice (absolute) ; path . push (0) ; } ,) ? ; Ok (path) }
}

macro_rules! absolute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function absolute in module {}", module_path!());
    };
}

mkfn!{
    absolute_introspect!();
    # [doc = " Make a Windows path absolute."] pub (crate) fn absolute (path : & Path) -> io :: Result < PathBuf > { let path = path . as_os_str () ; let prefix = parse_prefix (path) ; if prefix . map (| x | x . is_verbatim ()) . unwrap_or (false) { if path . as_encoded_bytes () . contains (& 0) { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "strings passed to WinAPI cannot contain NULs" ,)) ; } return Ok (path . to_owned () . into ()) ; } let path = to_u16s (path) ? ; let lpfilename = path . as_ptr () ; fill_utf16_buf (| buffer , size | unsafe { c :: GetFullPathNameW (lpfilename , size , buffer , ptr :: null_mut ()) } , os2path ,) }
}

macro_rules! is_absolute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_absolute in module {}", module_path!());
    };
}

mkfn!{
    is_absolute_introspect!();
    pub (crate) fn is_absolute (path : & Path) -> bool { path . has_root () && path . prefix () . is_some () }
}

macro_rules! is_absolute_exact_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_absolute_exact in module {}", module_path!());
    };
}

mkfn!{
    is_absolute_exact_introspect!();
    # [doc = " Test that the path is absolute, fully qualified and unchanged when processed by the Windows API."] # [doc = ""] # [doc = " For example:"] # [doc = ""] # [doc = " - `C:\\path\\to\\file` will return true."] # [doc = " - `C:\\path\\to\\nul` returns false because the Windows API will convert it to \\\\.\\NUL"] # [doc = " - `C:\\path\\to\\..\\file` returns false because it will be resolved to `C:\\path\\file`."] # [doc = ""] # [doc = " This is a useful property because it means the path can be converted from and to and verbatim"] # [doc = " path just by changing the prefix."] pub (crate) fn is_absolute_exact (path : & [u16]) -> bool { if path . is_empty () || path . len () > u32 :: MAX as usize || path . last () != Some (& 0) { return false ; } let buffer_len = path . len () ; let mut new_path = Vec :: with_capacity (buffer_len) ; let result = unsafe { c :: GetFullPathNameW (path . as_ptr () , new_path . capacity () as u32 , new_path . as_mut_ptr () , crate :: ptr :: null_mut () ,) } ; if result == 0 || result as usize != buffer_len - 1 { false } else { unsafe { new_path . set_len ((result as usize) + 1) ; } path == & new_path } }
}