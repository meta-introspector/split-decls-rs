// Generated macro for impl_143 (impl)
macro_rules! Depcrate_reducedimpl_143 {
() => {
// Module: crate::reduced
// Provides: {"impl_143"}
// Dependencies: {}
impl < T : PartialEq + Clone , R : Reducer < T > + Clone > Div < & ReducedInt < T , R > > for & ReducedInt < T , R > { type Output = ReducedInt < T , R > ; # [inline] fn div (self , rhs : & ReducedInt < T , R >) -> Self :: Output { self . check_modulus_eq (rhs) ; let a = self . r . mul (& self . a , & self . r . inv (rhs . a . clone ()) . expect (INV_ERR_MSG)) ; ReducedInt { a , r : self . r . clone () , } } }
};
}
