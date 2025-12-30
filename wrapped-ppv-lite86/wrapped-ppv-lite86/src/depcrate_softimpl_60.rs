// Generated macro for impl_60 (impl)
macro_rules! Depcrate_softimpl_60 {
() => {
// Module: crate::soft
// Provides: {"impl_60"}
// Dependencies: {}
impl < W : Not + Copy > Not for x4 < W > { type Output = x4 < W :: Output > ; # [inline (always)] fn not (self) -> Self :: Output { x4 ([self . 0 [0] . not () , self . 0 [1] . not () , self . 0 [2] . not () , self . 0 [3] . not () ,]) } }
};
}
