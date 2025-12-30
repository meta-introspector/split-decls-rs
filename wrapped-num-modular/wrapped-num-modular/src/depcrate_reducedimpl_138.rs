// Generated macro for impl_138 (impl)
macro_rules! Depcrate_reducedimpl_138 {
() => {
// Module: crate::reduced
// Provides: {"impl_138"}
// Dependencies: {}
# [cfg (feature = "num-traits")] impl < T : PartialEq , R : Reducer < T > > Inv for ReducedInt < T , R > { type Output = Self ; # [inline] fn inv (self) -> Self :: Output { self . inv () . expect (INV_ERR_MSG) } }
};
}
