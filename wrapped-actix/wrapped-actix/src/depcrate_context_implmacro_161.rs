// Generated macro for macro_161 (macro)
macro_rules! Depcrate_context_implmacro_161 {
() => {
// Module: crate::context_impl
// Provides: {"macro_161"}
// Dependencies: {}
bitflags ! { # [doc = " Internal context state."] # [derive (Debug)] struct ContextFlags : u8 { const STARTED = 0b0000_0001 ; const RUNNING = 0b0000_0010 ; const STOPPING = 0b0000_0100 ; const STOPPED = 0b0001_0000 ; const MB_CAP_CHANGED = 0b0010_0000 ; } }
};
}
