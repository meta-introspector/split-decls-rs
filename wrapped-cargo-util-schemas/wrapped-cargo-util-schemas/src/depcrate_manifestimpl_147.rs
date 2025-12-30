// Generated macro for impl_147 (impl)
macro_rules! Depcrate_manifestimpl_147 {
() => {
// Module: crate::manifest
// Provides: {"impl_147"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for InheritableRustVersion { fn deserialize < D > (d : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { struct Visitor ; impl < 'de > de :: Visitor < 'de > for Visitor { type Value = InheritableRustVersion ; fn expecting (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . write_str ("a semver or workspace") } fn visit_string < E > (self , value : String) -> Result < Self :: Value , E > where E : de :: Error , { let value = value . parse :: < RustVersion > () . map_err (| e | E :: custom (e)) ? ; Ok (InheritableRustVersion :: Value (value)) } fn visit_str < E > (self , value : & str) -> Result < Self :: Value , E > where E : de :: Error , { self . visit_string (value . to_owned ()) } fn visit_map < V > (self , map : V) -> Result < Self :: Value , V :: Error > where V : de :: MapAccess < 'de > , { let mvd = de :: value :: MapAccessDeserializer :: new (map) ; TomlInheritedField :: deserialize (mvd) . map (InheritableField :: Inherit) } } d . deserialize_any (Visitor) } }
};
}
