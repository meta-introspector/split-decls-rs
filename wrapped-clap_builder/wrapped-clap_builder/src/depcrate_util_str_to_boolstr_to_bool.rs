// Generated macro for str_to_bool (function)
macro_rules! Depcrate_util_str_to_boolstr_to_bool {
() => {
// Module: crate::util::str_to_bool
// Provides: {"str_to_bool"}
// Dependencies: {}
# [doc = " Converts a string literal representation of truth to true or false."] # [doc = ""] # [doc = " `false` values are `n`, `no`, `f`, `false`, `off`, and `0` (case insensitive)."] # [doc = ""] # [doc = " Any other value will be considered as `true`."] pub (crate) fn str_to_bool (val : impl AsRef < str >) -> Option < bool > { let pat : & str = & val . as_ref () . to_lowercase () ; if TRUE_LITERALS . contains (& pat) { Some (true) } else if FALSE_LITERALS . contains (& pat) { Some (false) } else { None } }
};
}
