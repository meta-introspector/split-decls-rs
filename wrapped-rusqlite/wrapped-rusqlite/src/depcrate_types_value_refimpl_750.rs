// Generated macro for impl_750 (impl)
macro_rules! Depcrate_types_value_refimpl_750 {
() => {
// Module: crate::types::value_ref
// Provides: {"impl_750"}
// Dependencies: {}
impl < 'a > From < & 'a str > for ValueRef < 'a > { # [inline] fn from (s : & str) -> ValueRef < '_ > { ValueRef :: Text (s . as_bytes ()) } }
};
}
