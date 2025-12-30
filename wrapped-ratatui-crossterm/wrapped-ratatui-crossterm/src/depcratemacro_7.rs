// Generated macro for macro_7 (macro)
macro_rules! Depcratemacro_7 {
() => {
// Module: crate
// Provides: {"macro_7"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (feature = "crossterm_0_29")] { pub use crossterm_0_29 as crossterm ; } else if # [cfg (feature = "crossterm_0_28")] { pub use crossterm_0_28 as crossterm ; } else { compile_error ! ("At least one crossterm feature must be enabled. See the crate docs for more information.") ; } }
};
}
