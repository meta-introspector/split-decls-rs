// Generated macro for impl_65 (impl)
macro_rules! Depcrate_paramsimpl_65 {
() => {
// Module: crate::params
// Provides: {"impl_65"}
// Dependencies: {}
impl AsRef < str > for Buffer { fn as_ref (& self) -> & str { str :: from_utf8 (& self . bytes [.. (self . length as usize)]) . expect (INVARIANT_VIOLATED_MSG) } }
};
}
