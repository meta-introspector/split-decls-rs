// Generated macro for impl_304 (impl)
macro_rules! Depcrate_stringimpl_304 {
() => {
// Module: crate::string
// Provides: {"impl_304"}
// Dependencies: {}
impl < LenT1 : LenType , LenT2 : LenType , S1 : StringStorage + ? Sized , S2 : StringStorage + ? Sized > PartialOrd < StringInner < LenT1 , S1 > > for StringInner < LenT2 , S2 > { # [inline] fn partial_cmp (& self , other : & StringInner < LenT1 , S1 >) -> Option < Ordering > { PartialOrd :: partial_cmp (& * * self , & * * other) } }
};
}
