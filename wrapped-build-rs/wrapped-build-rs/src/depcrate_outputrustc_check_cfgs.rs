// Generated macro for rustc_check_cfgs (function)
macro_rules! Depcrate_outputrustc_check_cfgs {
() => {
// Module: crate::output
// Provides: {"rustc_check_cfgs"}
// Dependencies: {}
# [doc = " Add to the list of expected config names that is used when checking the"] # [doc = " *reachable* cfg expressions with the [`unexpected_cfgs`] lint."] # [doc = ""] # [doc = " This form is for keys without an expected value, such as `cfg(name)`."] # [doc = ""] # [doc = " It is recommended to group the `rustc_check_cfg` and `rustc_cfg` calls as"] # [doc = " closely as possible in order to avoid typos, missing check_cfg, stale cfgs,"] # [doc = " and other mistakes."] # [doc = ""] # [doc = " [`unexpected_cfgs`]: https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#unexpected-cfgs"] # [doc = respected_msrv ! ("1.80")] # [track_caller] pub fn rustc_check_cfgs (keys : & [& str]) { if keys . is_empty () { return ; } for key in keys { if ! is_ident (key) { panic ! ("cannot emit rustc-check-cfg: invalid key {key:?}") ; } } let mut directive = keys [0] . to_string () ; for key in & keys [1 ..] { write ! (directive , ", {key}") . expect ("writing to string should be infallible") ; } emit ("rustc-check-cfg" , format_args ! ("cfg({directive})")) ; }
};
}
