// Generated macro for impl_115 (impl)
macro_rules! Depcrate_boxedimpl_115 {
() => {
// Module: crate::boxed
// Provides: {"impl_115"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < T , A > serde :: Serialize for Box < T , A > where T : serde :: Serialize , A : Allocator , { # [inline (always)] fn serialize < S : serde :: ser :: Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { (* * self) . serialize (serializer) } }
};
}
