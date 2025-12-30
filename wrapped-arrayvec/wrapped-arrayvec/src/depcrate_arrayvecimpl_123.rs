// Generated macro for impl_123 (impl)
macro_rules! Depcrate_arrayvecimpl_123 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_123"}
// Dependencies: {}
# [cfg (feature = "serde")] # [doc = " Requires crate feature `\"serde\"`"] impl < T : Serialize , const CAP : usize > Serialize for ArrayVec < T , CAP > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_seq (self) } }
};
}
