// Generated macro for impl_139 (impl)
macro_rules! Depcrate_reducedimpl_139 {
() => {
// Module: crate::reduced
// Provides: {"impl_139"}
// Dependencies: {}
# [cfg (feature = "num-traits")] impl < T : PartialEq + Clone , R : Reducer < T > + Clone > Inv for & ReducedInt < T , R > { type Output = ReducedInt < T , R > ; # [inline] fn inv (self) -> Self :: Output { self . clone () . inv () . expect (INV_ERR_MSG) } }
};
}
