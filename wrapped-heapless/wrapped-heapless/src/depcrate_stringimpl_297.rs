// Generated macro for impl_297 (impl)
macro_rules! Depcrate_stringimpl_297 {
() => {
// Module: crate::string
// Provides: {"impl_297"}
// Dependencies: {}
impl < LenT : LenType , S : StringStorage + ? Sized > AsRef < [u8] > for StringInner < LenT , S > { # [inline] fn as_ref (& self) -> & [u8] { self . as_bytes () } }
};
}
