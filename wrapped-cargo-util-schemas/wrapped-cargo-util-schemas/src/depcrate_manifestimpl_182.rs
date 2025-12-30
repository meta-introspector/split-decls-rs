// Generated macro for impl_182 (impl)
macro_rules! Depcrate_manifestimpl_182 {
() => {
// Module: crate::manifest
// Provides: {"impl_182"}
// Dependencies: {}
impl ser :: Serialize for TomlOptLevel { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { match self . 0 . parse :: < u32 > () { Ok (n) => n . serialize (serializer) , Err (_) => self . 0 . serialize (serializer) , } } }
};
}
