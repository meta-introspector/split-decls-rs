// Generated macro for impl_48 (impl)
macro_rules! Depcrate_integerimpl_48 {
() => {
// Module: crate::integer
// Provides: {"impl_48"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde :: Serialize for Suffix { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serializer . serialize_str (match self { Self :: Kibi => "k" , Self :: Mebi => "m" , Self :: Gibi => "g" , }) } }
};
}
