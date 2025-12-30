// Generated macro for LDR_IS_RESOURCE (function)
macro_rules! Depcrate_ntldrLDR_IS_RESOURCE {
() => {
// Module: crate::ntldr
// Provides: {"LDR_IS_RESOURCE"}
// Dependencies: {}
# [inline] pub const fn LDR_IS_RESOURCE (DllHandle : ULONG_PTR) -> bool { LDR_IS_IMAGEMAPPING (DllHandle) || LDR_IS_DATAFILE (DllHandle) }
};
}
