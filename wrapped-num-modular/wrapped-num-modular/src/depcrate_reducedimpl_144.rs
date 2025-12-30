// Generated macro for impl_144 (impl)
macro_rules! Depcrate_reducedimpl_144 {
() => {
// Module: crate::reduced
// Provides: {"impl_144"}
// Dependencies: {}
# [cfg (feature = "num-traits")] impl < T : PartialEq , R : Reducer < T > > Pow < T > for ReducedInt < T , R > { type Output = Self ; # [inline] fn pow (self , rhs : T) -> Self :: Output { ReducedInt :: pow (self , & rhs) } }
};
}
