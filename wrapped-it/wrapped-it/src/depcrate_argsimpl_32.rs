// Generated macro for impl_32 (impl)
macro_rules! Depcrate_argsimpl_32 {
() => {
// Module: crate::args
// Provides: {"impl_32"}
// Dependencies: {}
impl TypedValueParser for AsPathSpec { type Value = gix :: pathspec :: Pattern ; fn parse_ref (& self , cmd : & Command , arg : Option < & Arg > , value : & OsStr) -> Result < Self :: Value , Error > { let pathspec_defaults = gix :: pathspec :: Defaults :: from_environment (& mut | n | std :: env :: var_os (n)) . unwrap_or_default () ; OsStringValueParser :: new () . try_map (move | arg | { let arg : & std :: path :: Path = arg . as_os_str () . as_ref () ; gix :: pathspec :: parse (gix :: path :: into_bstr (arg) . as_ref () , pathspec_defaults) }) . parse_ref (cmd , arg , value) } }
};
}
