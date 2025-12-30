// Generated macro for impl_293 (impl)
macro_rules! Depcrate_stringimpl_293 {
() => {
// Module: crate::string
// Provides: {"impl_293"}
// Dependencies: {}
impl < LenT : LenType , S : StringStorage + ? Sized > ops :: DerefMut for StringInner < LenT , S > { fn deref_mut (& mut self) -> & mut str { self . as_mut_str () } }
};
}
