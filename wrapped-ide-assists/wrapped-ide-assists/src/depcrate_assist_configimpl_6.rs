// Generated macro for impl_6 (impl)
macro_rules! Depcrate_assist_configimpl_6 {
() => {
// Module: crate::assist_config
// Provides: {"impl_6"}
// Dependencies: {}
impl AssistConfig { pub fn import_path_config (& self) -> ImportPathConfig { ImportPathConfig { prefer_no_std : self . prefer_no_std , prefer_prelude : self . prefer_prelude , prefer_absolute : self . prefer_absolute , } } pub fn find_path_config (& self , allow_unstable : bool) -> FindPathConfig { FindPathConfig { prefer_no_std : self . prefer_no_std , prefer_prelude : self . prefer_prelude , prefer_absolute : self . prefer_absolute , allow_unstable , } } }
};
}
