// Generated macro for impl_237 (impl)
macro_rules! Depcrate_manifestimpl_237 {
() => {
// Module: crate::manifest
// Provides: {"impl_237"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for TomlPackageBuild { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { UntaggedEnumVisitor :: new () . bool (| b | Ok (TomlPackageBuild :: Auto (b))) . string (| s | Ok (TomlPackageBuild :: SingleScript (s . to_owned ()))) . seq (| value | value . deserialize () . map (TomlPackageBuild :: MultipleScript)) . deserialize (deserializer) } }
};
}
