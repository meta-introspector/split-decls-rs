// Generated macro for LDR_IS_DATAFILE (function)
macro_rules! Depcrate_ntldrLDR_IS_DATAFILE {
() => {
// Module: crate::ntldr
// Provides: {"LDR_IS_DATAFILE"}
// Dependencies: {}
# [inline] pub const fn LDR_IS_DATAFILE (DllHandle : ULONG_PTR) -> bool { DllHandle & 1 != 0 }
};
}
