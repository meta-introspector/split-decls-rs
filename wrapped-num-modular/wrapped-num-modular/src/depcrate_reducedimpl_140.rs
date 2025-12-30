// Generated macro for impl_140 (impl)
macro_rules! Depcrate_reducedimpl_140 {
() => {
// Module: crate::reduced
// Provides: {"impl_140"}
// Dependencies: {}
impl < T : PartialEq , R : Reducer < T > > Div for ReducedInt < T , R > { type Output = Self ; # [inline] fn div (self , rhs : Self) -> Self :: Output { self . check_modulus_eq (& rhs) ; let ReducedInt { a , r } = rhs ; let a = r . mul (& self . a , & r . inv (a) . expect (INV_ERR_MSG)) ; ReducedInt { a , r } } }
};
}
