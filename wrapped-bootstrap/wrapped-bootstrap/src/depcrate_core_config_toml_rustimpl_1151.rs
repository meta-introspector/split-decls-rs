// Generated macro for impl_1151 (impl)
macro_rules! Depcrate_core_config_toml_rustimpl_1151 {
() => {
// Module: crate::core::config::toml::rust
// Provides: {"impl_1151"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for LldMode { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct LldModeVisitor ; impl serde :: de :: Visitor < '_ > for LldModeVisitor { type Value = LldMode ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { formatter . write_str ("one of true, 'self-contained' or 'external'") } fn visit_bool < E > (self , v : bool) -> Result < Self :: Value , E > where E : serde :: de :: Error , { Ok (if v { LldMode :: External } else { LldMode :: Unused }) } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : serde :: de :: Error , { match v { "external" => Ok (LldMode :: External) , "self-contained" => Ok (LldMode :: SelfContained) , _ => Err (E :: custom (format ! ("unknown mode {v}"))) , } } } deserializer . deserialize_any (LldModeVisitor) } }
};
}
