// Generated macro for impl_145 (impl)
macro_rules! Depcrate_reducedimpl_145 {
() => {
// Module: crate::reduced
// Provides: {"impl_145"}
// Dependencies: {}
# [cfg (feature = "num-traits")] impl < T : PartialEq + Clone , R : Reducer < T > + Clone > Pow < T > for & ReducedInt < T , R > { type Output = ReducedInt < T , R > ; # [inline] fn pow (self , rhs : T) -> Self :: Output { let a = self . r . pow (self . a . clone () , & rhs) ; ReducedInt { a , r : self . r . clone () , } } }
};
}
