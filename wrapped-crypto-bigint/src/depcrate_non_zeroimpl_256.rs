// Generated macro for impl_256 (impl)
macro_rules! Depcrate_non_zeroimpl_256 {
() => {
// Module: crate::non_zero
// Provides: {"impl_256"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < T : Serialize + Zero > Serialize for NonZero < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (serializer) } }
};
}
