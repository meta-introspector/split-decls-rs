// Generated macro for impl_294 (impl)
macro_rules! Depcrate_stringimpl_294 {
() => {
// Module: crate::string
// Provides: {"impl_294"}
// Dependencies: {}
impl < LenT : LenType , S : StringStorage + ? Sized > borrow :: Borrow < str > for StringInner < LenT , S > { fn borrow (& self) -> & str { self . as_str () } }
};
}
