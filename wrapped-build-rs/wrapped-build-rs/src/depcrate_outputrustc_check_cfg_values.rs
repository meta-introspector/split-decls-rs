// Generated macro for rustc_check_cfg_values (function)
macro_rules! Depcrate_outputrustc_check_cfg_values {
() => {
// Module: crate::output
// Provides: {"rustc_check_cfg_values"}
// Dependencies: {}
# [doc = " Add to the list of expected config names that is used when checking the"] # [doc = " *reachable* cfg expressions with the [`unexpected_cfgs`] lint."] # [doc = ""] # [doc = " This form is for keys with expected values, such as `cfg(name = \"value\")`."] # [doc = ""] # [doc = " It is recommended to group the `rustc_check_cfg` and `rustc_cfg` calls as"] # [doc = " closely as possible in order to avoid typos, missing check_cfg, stale cfgs,"] # [doc = " and other mistakes."] # [doc = ""] # [doc = " [`unexpected_cfgs`]: https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#unexpected-cfgs"] # [doc = respected_msrv ! ("1.80")] # [track_caller] pub fn rustc_check_cfg_values (key : & str , values : & [& str]) { if ! is_ident (key) { panic ! ("cannot emit rustc-check-cfg: invalid key {key:?}") ; } if values . is_empty () { rustc_check_cfgs (& [key]) ; return ; } let mut directive = format ! ("\"{}\"" , values [0] . escape_default ()) ; for value in & values [1 ..] { write ! (directive , ", \"{}\"" , value . escape_default ()) . expect ("writing to string should be infallible") ; } emit ("rustc-check-cfg" , format_args ! ("cfg({key}, values({directive}))") ,) ; }
};
}
