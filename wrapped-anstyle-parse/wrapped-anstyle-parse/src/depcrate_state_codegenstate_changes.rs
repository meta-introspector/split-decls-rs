// Generated macro for STATE_CHANGES (static)
macro_rules! Depcrate_state_codegenSTATE_CHANGES {
() => {
// Module: crate::state::codegen
// Provides: {"STATE_CHANGES"}
// Dependencies: {}
# [doc = " This is the state change table. It's indexed first by current state and then by the next"] # [doc = " character in the pty stream."] pub (crate) static STATE_CHANGES : [[u8 ; 256] ; 16] = state_changes () ;
};
}
