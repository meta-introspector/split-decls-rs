// Generated macro for flag (function)
macro_rules! Depcrate_pipeflag {
() => {
// Module: crate::pipe
// Provides: {"flag"}
// Dependencies: {}
fn flag (slot : & mut u32 , on : bool , val : u32) { if on { * slot |= val ; } else { * slot &= ! val ; } }
};
}
