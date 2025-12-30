// Generated macro for impl_280 (impl)
macro_rules! Depcrate_vecimpl_280 {
() => {
// Module: crate::vec
// Provides: {"impl_280"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < T , A > serde :: Serialize for Vec < T , A > where T : serde :: Serialize , A : Allocator , { # [inline (always)] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { serializer . collect_seq (self) } }
};
}
