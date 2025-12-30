// Generated macro for impl_724 (impl)
macro_rules! Depcrate_numberimpl_724 {
() => {
// Module: crate::number
// Provides: {"impl_724"}
// Dependencies: {}
# [cfg (feature = "arbitrary_precision")] impl < 'de > de :: Deserialize < 'de > for NumberFromString { fn deserialize < D > (deserializer : D) -> Result < NumberFromString , D :: Error > where D : de :: Deserializer < 'de > , { struct Visitor ; impl < 'de > de :: Visitor < 'de > for Visitor { type Value = NumberFromString ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("string containing a number") } fn visit_str < E > (self , s : & str) -> Result < NumberFromString , E > where E : de :: Error , { let n = tri ! (s . parse () . map_err (de :: Error :: custom)) ; Ok (NumberFromString { value : n }) } } deserializer . deserialize_str (Visitor) } }
};
}
