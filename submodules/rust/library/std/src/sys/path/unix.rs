mkuse!{use crate :: ffi :: OsStr ;}
mkuse!{use crate :: path :: { Path , PathBuf , Prefix } ;}
mkuse!{use crate :: { env , io } ;}

macro_rules! is_sep_byte_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_sep_byte in module {}", module_path!());
    };
}

mkfn!{
    is_sep_byte_introspect!();
    # [inline] pub fn is_sep_byte (b : u8) -> bool { b == b'/' }
}

macro_rules! is_verbatim_sep_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_verbatim_sep in module {}", module_path!());
    };
}

mkfn!{
    is_verbatim_sep_introspect!();
    # [inline] pub fn is_verbatim_sep (b : u8) -> bool { b == b'/' }
}

macro_rules! parse_prefix_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_prefix in module {}", module_path!());
    };
}

mkfn!{
    parse_prefix_introspect!();
    # [inline] pub fn parse_prefix (_ : & OsStr) -> Option < Prefix < '_ > > { None }
}
mkitem!{pub const MAIN_SEP_STR : & str = "/" ;}
mkitem!{pub const MAIN_SEP : char = '/' ;}

macro_rules! absolute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function absolute in module {}", module_path!());
    };
}

mkfn!{
    absolute_introspect!();
    # [doc = " Make a POSIX path absolute without changing its semantics."] pub (crate) fn absolute (path : & Path) -> io :: Result < PathBuf > { let mut components = path . strip_prefix (".") . unwrap_or (path) . components () ; let path_os = path . as_os_str () . as_encoded_bytes () ; let mut normalized = if path . is_absolute () { if path_os . starts_with (b"//") && ! path_os . starts_with (b"///") { components . next () ; PathBuf :: from ("//") } else { PathBuf :: new () } } else { env :: current_dir () ? } ; normalized . extend (components) ; if path_os . ends_with (b"/") { normalized . push ("") ; } Ok (normalized) }
}

macro_rules! is_absolute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_absolute in module {}", module_path!());
    };
}

mkfn!{
    is_absolute_introspect!();
    pub (crate) fn is_absolute (path : & Path) -> bool { if cfg ! (any (unix , target_os = "hermit" , target_os = "wasi")) { path . has_root () } else { path . has_root () && path . prefix () . is_some () } }
}