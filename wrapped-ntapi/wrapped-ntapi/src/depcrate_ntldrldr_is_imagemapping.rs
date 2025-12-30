// Generated macro for LDR_IS_IMAGEMAPPING (function)
macro_rules! Depcrate_ntldrLDR_IS_IMAGEMAPPING {
() => {
// Module: crate::ntldr
// Provides: {"LDR_IS_IMAGEMAPPING"}
// Dependencies: {}
# [inline] pub const fn LDR_IS_IMAGEMAPPING (DllHandle : ULONG_PTR) -> bool { DllHandle & 2 != 0 }
};
}
