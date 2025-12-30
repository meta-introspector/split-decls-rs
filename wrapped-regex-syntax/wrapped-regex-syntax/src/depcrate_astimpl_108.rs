// Generated macro for impl_108 (impl)
macro_rules! Depcrate_astimpl_108 {
() => {
// Module: crate::ast
// Provides: {"impl_108"}
// Dependencies: {}
impl ClassSetRange { # [doc = " Returns true if and only if this character class range is valid."] # [doc = ""] # [doc = " The only case where a range is invalid is if its start is greater than"] # [doc = " its end."] pub fn is_valid (& self) -> bool { self . start . c <= self . end . c } }
};
}
