// Generated macro for impl_292 (impl)
macro_rules! Depcrate_stringimpl_292 {
() => {
// Module: crate::string
// Provides: {"impl_292"}
// Dependencies: {}
impl < LenT : LenType , S : StringStorage + ? Sized > ops :: Deref for StringInner < LenT , S > { type Target = str ; fn deref (& self) -> & str { self . as_str () } }
};
}
