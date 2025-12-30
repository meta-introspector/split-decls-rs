// Generated macro for impl_761 (impl)
macro_rules! Depcrate_ir_functionimpl_761 {
() => {
// Module: crate::ir::function
// Provides: {"impl_761"}
// Dependencies: {}
# [cfg (feature = "enable-serde")] impl < 'de > Deserialize < 'de > for VersionMarker { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let version = String :: deserialize (deserializer) ? ; if version != crate :: VERSION { return Err (D :: Error :: custom (& format ! ("Expected a clif ir function for version {}, found one for version {}" , crate :: VERSION , version ,))) ; } Ok (VersionMarker) } }
};
}
