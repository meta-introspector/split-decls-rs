// Generated macro for impl_305 (impl)
macro_rules! Depcrate_oddimpl_305 {
() => {
// Module: crate::odd
// Provides: {"impl_305"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < T : Serialize + Zero > Serialize for Odd < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (serializer) } }
};
}
