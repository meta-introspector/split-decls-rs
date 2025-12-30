// Generated macro for impl_295 (impl)
macro_rules! Depcrate_stringimpl_295 {
() => {
// Module: crate::string
// Provides: {"impl_295"}
// Dependencies: {}
impl < LenT : LenType , S : StringStorage + ? Sized > borrow :: BorrowMut < str > for StringInner < LenT , S > { fn borrow_mut (& mut self) -> & mut str { self . as_mut_str () } }
};
}
