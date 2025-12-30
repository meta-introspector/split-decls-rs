// Generated macro for macro_96 (macro)
macro_rules! Depcrate_random_statemacro_96 {
() => {
// Module: crate::random_state
// Provides: {"macro_96"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (not (all (target_arch = "arm" , target_os = "none")))] { use once_cell :: race :: OnceBox ; static RAND_SOURCE : OnceBox < Box < dyn RandomSource + Send + Sync >> = OnceBox :: new () ; } }
};
}
