// Generated macro for impl_142 (impl)
macro_rules! Depcrate_reducedimpl_142 {
() => {
// Module: crate::reduced
// Provides: {"impl_142"}
// Dependencies: {}
impl < T : PartialEq + Clone , R : Reducer < T > > Div < ReducedInt < T , R > > for & ReducedInt < T , R > { type Output = ReducedInt < T , R > ; # [inline] fn div (self , rhs : ReducedInt < T , R >) -> Self :: Output { self . check_modulus_eq (& rhs) ; let ReducedInt { a , r } = rhs ; let a = r . mul (& self . a , & r . inv (a) . expect (INV_ERR_MSG)) ; ReducedInt { a , r } } }
};
}
