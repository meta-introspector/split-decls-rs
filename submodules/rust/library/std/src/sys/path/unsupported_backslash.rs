mkuse!{use crate :: ffi :: OsStr ;}
mkuse!{use crate :: io ;}
mkuse!{use crate :: path :: { Path , PathBuf , Prefix } ;}
mkuse!{use crate :: sys :: unsupported ;}

macro_rules! is_sep_byte_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_sep_byte in module {}", module_path!());
    };
}

mkfn!{
    is_sep_byte_introspect!();
    # [inline] pub fn is_sep_byte (b : u8) -> bool { b == b'\\' }
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

macro_rules! parse_prefix_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_prefix in module {}", module_path!());
    };
}

mkfn!{
    parse_prefix_introspect!();
    pub fn parse_prefix (_ : & OsStr) -> Option < Prefix < '_ > > { None }
}
mkitem!{pub const MAIN_SEP_STR : & str = "\\" ;}
mkitem!{pub const MAIN_SEP : char = '\\' ;}

macro_rules! absolute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function absolute in module {}", module_path!());
    };
}

mkfn!{
    absolute_introspect!();
    pub (crate) fn absolute (_path : & Path) -> io :: Result < PathBuf > { unsupported () }
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