// Generated macro for impl_151 (impl)
macro_rules! Depcrate_manifestimpl_151 {
() => {
// Module: crate::manifest
// Provides: {"impl_151"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for InheritableStringOrBool { fn deserialize < D > (d : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { struct Visitor ; impl < 'de > de :: Visitor < 'de > for Visitor { type Value = InheritableStringOrBool ; fn expecting (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str ("a string, a bool, or workspace") } fn visit_bool < E > (self , v : bool) -> Result < Self :: Value , E > where E : de :: Error , { let b = de :: value :: BoolDeserializer :: new (v) ; StringOrBool :: deserialize (b) . map (InheritableField :: Value) } fn visit_string < E > (self , v : String) -> Result < Self :: Value , E > where E : de :: Error , { let string = de :: value :: StringDeserializer :: new (v) ; StringOrBool :: deserialize (string) . map (InheritableField :: Value) } fn visit_str < E > (self , value : & str) -> Result < Self :: Value , E > where E : de :: Error , { self . visit_string (value . to_owned ()) } fn visit_map < V > (self , map : V) -> Result < Self :: Value , V :: Error > where V : de :: MapAccess < 'de > , { let mvd = de :: value :: MapAccessDeserializer :: new (map) ; TomlInheritedField :: deserialize (mvd) . map (InheritableField :: Inherit) } } d . deserialize_any (Visitor) } }
};
}
