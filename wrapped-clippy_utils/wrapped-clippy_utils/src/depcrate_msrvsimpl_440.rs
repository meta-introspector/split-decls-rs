// Generated macro for impl_440 (impl)
macro_rules! Depcrate_msrvsimpl_440 {
() => {
// Module: crate::msrvs
// Provides: {"impl_440"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Msrv { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let v = String :: deserialize (deserializer) ? ; parse_version (Symbol :: intern (& v)) . map (| v | Self (Some (v))) . ok_or_else (| | serde :: de :: Error :: custom ("not a valid Rust version")) } }
};
}
