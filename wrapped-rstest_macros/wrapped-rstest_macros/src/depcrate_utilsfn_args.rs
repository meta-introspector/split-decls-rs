// Generated macro for fn_args (function)
macro_rules! Depcrate_utilsfn_args {
() => {
// Module: crate::utils
// Provides: {"fn_args"}
// Dependencies: {}
# [doc = " Return an iterator over fn arguments."] # [doc = ""] pub (crate) fn fn_args (item_fn : & ItemFn) -> impl Iterator < Item = & FnArg > { item_fn . sig . inputs . iter () }
};
}
