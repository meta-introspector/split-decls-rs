// Generated macro for macro_63 (macro)
macro_rules! Depcratemacro_63 {
() => {
// Module: crate
// Provides: {"macro_63"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (feature = "std")] { pub use self :: backtrace :: trace ; pub use self :: symbolize :: { resolve , resolve_frame } ; pub use self :: capture :: { Backtrace , BacktraceFrame , BacktraceSymbol } ; mod capture ; } }
};
}
