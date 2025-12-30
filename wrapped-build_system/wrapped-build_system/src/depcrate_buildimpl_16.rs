// Generated macro for impl_16 (impl)
macro_rules! Depcrate_buildimpl_16 {
() => {
// Module: crate::build
// Provides: {"impl_16"}
// Dependencies: {}
impl BuildArg { # [doc = " Creates a new `BuildArg` instance by parsing command-line arguments."] fn new () -> Result < Option < Self > , String > { let mut build_arg = Self :: default () ; let mut args = std :: env :: args () . skip (2) ; while let Some (arg) = args . next () { match arg . as_str () { "--sysroot" => { build_arg . build_sysroot = true ; } "--help" => { Self :: usage () ; return Ok (None) ; } arg => { if ! build_arg . config_info . parse_argument (arg , & mut args) ? { return Err (format ! ("Unknown argument `{arg}`")) ; } } } } Ok (Some (build_arg)) } fn usage () { println ! (r#"
`build` command help:

    --sysroot              : Build with sysroot"#) ; ConfigInfo :: show_usage () ; println ! ("    --help                 : Show this help") ; } }
};
}
