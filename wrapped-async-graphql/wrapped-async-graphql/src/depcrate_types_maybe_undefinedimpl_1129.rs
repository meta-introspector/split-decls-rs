// Generated macro for impl_1129 (impl)
macro_rules! Depcrate_types_maybe_undefinedimpl_1129 {
() => {
// Module: crate::types::maybe_undefined
// Provides: {"impl_1129"}
// Dependencies: {}
impl < 'de , T > Deserialize < 'de > for MaybeUndefined < T > where T : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < MaybeUndefined < T > , D :: Error > where D : Deserializer < 'de > , { Option :: < T > :: deserialize (deserializer) . map (| value | match value { Some (value) => MaybeUndefined :: Value (value) , None => MaybeUndefined :: Null , }) } }
};
}
