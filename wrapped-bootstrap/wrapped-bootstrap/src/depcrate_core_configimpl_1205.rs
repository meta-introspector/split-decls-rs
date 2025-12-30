// Generated macro for impl_1205 (impl)
macro_rules! Depcrate_core_configimpl_1205 {
() => {
// Module: crate::core::config
// Provides: {"impl_1205"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for CompilerBuiltins { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (match Deserialize :: deserialize (deserializer) ? { StringOrBool :: Bool (false) => Self :: BuildRustOnly , StringOrBool :: Bool (true) => Self :: BuildLLVMFuncs , StringOrBool :: String (path) => Self :: LinkLLVMBuiltinsLib (path) , }) } }
};
}
