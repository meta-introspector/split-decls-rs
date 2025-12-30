// Generated macro for impl_149 (impl)
macro_rules! Depcrate_manifestimpl_149 {
() => {
// Module: crate::manifest
// Provides: {"impl_149"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for InheritableVecString { fn deserialize < D > (d : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { struct Visitor ; impl < 'de > de :: Visitor < 'de > for Visitor { type Value = InheritableVecString ; fn expecting (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str ("a vector of strings or workspace") } fn visit_seq < A > (self , v : A) -> Result < Self :: Value , A :: Error > where A : de :: SeqAccess < 'de > , { let seq = de :: value :: SeqAccessDeserializer :: new (v) ; Vec :: deserialize (seq) . map (InheritableField :: Value) } fn visit_map < V > (self , map : V) -> Result < Self :: Value , V :: Error > where V : de :: MapAccess < 'de > , { let mvd = de :: value :: MapAccessDeserializer :: new (map) ; TomlInheritedField :: deserialize (mvd) . map (InheritableField :: Inherit) } } d . deserialize_any (Visitor) } }
};
}
