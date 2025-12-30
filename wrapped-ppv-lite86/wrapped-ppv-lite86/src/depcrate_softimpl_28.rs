// Generated macro for impl_28 (impl)
macro_rules! Depcrate_softimpl_28 {
() => {
// Module: crate::soft
// Provides: {"impl_28"}
// Dependencies: {}
impl < W : Not + Copy , G > Not for x2 < W , G > { type Output = x2 < W :: Output , G > ; # [inline (always)] fn not (self) -> Self :: Output { x2 :: new ([self . 0 [0] . not () , self . 0 [1] . not ()]) } }
};
}
