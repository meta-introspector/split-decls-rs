// Generated macro for GDI_MAKE_HANDLE (function)
macro_rules! Depcrate_ntgdiGDI_MAKE_HANDLE {
() => {
// Module: crate::ntgdi
// Provides: {"GDI_MAKE_HANDLE"}
// Dependencies: {}
# [inline] pub const fn GDI_MAKE_HANDLE (Index : ULONG , Unique : ULONG) -> ULONG { Unique << GDI_HANDLE_INDEX_BITS | Index }
};
}
