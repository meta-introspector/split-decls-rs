// Generated macro for impl_47 (impl)
macro_rules! Depcrateimpl_47 {
() => {
// Module: crate
// Provides: {"impl_47"}
// Dependencies: {}
impl serde_core :: Serialize for Platform { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : serde_core :: Serializer , { self . to_string () . serialize (s) } }
};
}
