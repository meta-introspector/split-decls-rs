// Generated macro for impl_776 (impl)
macro_rules! Depcrate_base_conversionimpl_776 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_776"}
// Dependencies: {}
impl < T : Scalar , const R : usize , const C : usize > From < SMatrix < T , R , C > > for [[T ; R] ; C] { # [inline] fn from (mat : SMatrix < T , R , C >) -> Self { mat . data . 0 } }
};
}
