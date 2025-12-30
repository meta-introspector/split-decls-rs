// Generated macro for impl_44 (impl)
macro_rules! Depcrate_serdeimpl_44 {
() => {
// Module: crate::serde
// Provides: {"impl_44"}
// Dependencies: {}
impl < T , S > Serialize for IndexSet < T , S > where T : Serialize , { fn serialize < Se > (& self , serializer : Se) -> Result < Se :: Ok , Se :: Error > where Se : Serializer , { serializer . collect_seq (self) } }
};
}
