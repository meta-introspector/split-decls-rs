// Generated macro for macro_2 (macro)
macro_rules! Depcratemacro_2 {
() => {
// Module: crate
// Provides: {"macro_2"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (unix)] { use signal_hook_registry as registry ; } else if # [cfg (windows)] { mod windows_registry ; use windows_registry as registry ; } }
};
}
