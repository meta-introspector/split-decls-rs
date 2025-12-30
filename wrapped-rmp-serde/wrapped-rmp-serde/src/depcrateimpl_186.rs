// Generated macro for impl_186 (impl)
macro_rules! Depcrateimpl_186 {
() => {
// Module: crate
// Provides: {"impl_186"}
// Dependencies: {}
impl Serialize for Raw { fn serialize < S > (& self , se : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { match self . s { Ok (ref s) => se . serialize_str (s) , Err ((ref b , ..)) => se . serialize_bytes (b) , } } }
};
}
