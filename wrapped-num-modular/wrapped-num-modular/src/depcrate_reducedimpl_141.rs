// Generated macro for impl_141 (impl)
macro_rules! Depcrate_reducedimpl_141 {
() => {
// Module: crate::reduced
// Provides: {"impl_141"}
// Dependencies: {}
impl < T : PartialEq + Clone , R : Reducer < T > > Div < & ReducedInt < T , R > > for ReducedInt < T , R > { type Output = Self ; # [inline] fn div (self , rhs : & Self) -> Self :: Output { self . check_modulus_eq (rhs) ; let Self { a , r } = self ; let a = r . mul (& a , & r . inv (rhs . a . clone ()) . expect (INV_ERR_MSG)) ; ReducedInt { a , r } } }
};
}
