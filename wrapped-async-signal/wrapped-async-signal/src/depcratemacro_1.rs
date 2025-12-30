// Generated macro for macro_1 (macro)
macro_rules! Depcratemacro_1 {
() => {
// Module: crate
// Provides: {"macro_1"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (windows)] { mod channel ; use channel as sys ; } else { mod pipe ; use pipe as sys ; } }
};
}
