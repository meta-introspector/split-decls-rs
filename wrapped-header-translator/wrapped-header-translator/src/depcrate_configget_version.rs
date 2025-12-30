// Generated macro for get_version (function)
macro_rules! Depcrate_configget_version {
() => {
// Module: crate::config
// Provides: {"get_version"}
// Dependencies: {}
fn get_version < 'de , D : Deserializer < 'de > > (deserializer : D) -> Result < Option < Version > , D :: Error > { struct VersionVisitor ; impl de :: Visitor < '_ > for VersionVisitor { type Value = Option < Version > ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a version string") } fn visit_none < E > (self) -> Result < Self :: Value , E > where E : de :: Error , { Ok (None) } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : de :: Error , { Ok (Some (lenient_semver_parser :: parse :: < Version > (v) . map_err (de :: Error :: custom) ? ,)) } } deserializer . deserialize_str (VersionVisitor) }
};
}
