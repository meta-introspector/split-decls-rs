// Generated macro for fn_args_pats (function)
macro_rules! Depcrate_utilsfn_args_pats {
() => {
// Module: crate::utils
// Provides: {"fn_args_pats"}
// Dependencies: {}
# [doc = " Return an iterator over fn arguments items."] # [doc = ""] pub (crate) fn fn_args_pats (test : & ItemFn) -> impl Iterator < Item = & Pat > { fn_args (test) . filter_map (MaybePat :: maybe_pat) }
};
}
