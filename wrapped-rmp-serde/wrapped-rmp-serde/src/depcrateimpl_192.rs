// Generated macro for impl_192 (impl)
macro_rules! Depcrateimpl_192 {
() => {
// Module: crate
// Provides: {"impl_192"}
// Dependencies: {}
impl Serialize for RawRef < '_ > { fn serialize < S > (& self , se : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { match self . s { Ok (s) => se . serialize_str (s) , Err ((b , ..)) => se . serialize_bytes (b) , } } }
};
}
