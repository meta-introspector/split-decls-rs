// Generated macro for impl_243 (impl)
macro_rules! Depcrate_manifestimpl_243 {
() => {
// Module: crate::manifest
// Provides: {"impl_243"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for PathValue { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { Ok (PathValue (String :: deserialize (deserializer) ? . into ())) } }
};
}
