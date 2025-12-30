// Generated macro for __available_version (macro)
macro_rules! Depcrate___macros_available__available_version {
() => {
// Module: crate::__macros::available
// Provides: {"__available_version"}
// Dependencies: {}
# [doc = " Both `tt` and `literal` matches either `$major` as an integer, or"] # [doc = " `$major.$minor` as a float."] # [doc = ""] # [doc = " As such, we cannot just take `$major:tt . $minor:tt . $patch:tt` and"] # [doc = " convert that to `OSVersion` directly, we must convert it to a string"] # [doc = " first, and then parse that."] # [doc = ""] # [doc = " We also _have_ to do string parsing, floating point parsing wouldn't be"] # [doc = " enough (because e.g. `10.10` would result in the float `10.1` and parse"] # [doc = " wrongly)."] # [doc = ""] # [doc = " Note that we intentionally `stringify!` before passing to `concat!`, as"] # [doc = " that seems to properly preserve all zeros in the literal."] # [doc (hidden)] # [macro_export] macro_rules ! __available_version { ($ ($ version_part_or_period : tt) *) => { $ crate :: __macros :: OSVersion :: from_str ($ crate :: __macros :: concat ! ($ ($ crate :: __macros :: stringify ! ($ version_part_or_period) ,) *)) } ; }
};
}
