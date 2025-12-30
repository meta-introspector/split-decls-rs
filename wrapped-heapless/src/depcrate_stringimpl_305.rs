// Generated macro for impl_305 (impl)
macro_rules! Depcrate_stringimpl_305 {
() => {
// Module: crate::string
// Provides: {"impl_305"}
// Dependencies: {}
impl < LenT : LenType , S : StringStorage + ? Sized > Ord for StringInner < LenT , S > { # [inline] fn cmp (& self , other : & Self) -> Ordering { Ord :: cmp (& * * self , & * * other) } }
};
}
