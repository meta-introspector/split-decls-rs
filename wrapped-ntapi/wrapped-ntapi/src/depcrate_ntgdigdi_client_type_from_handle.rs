// Generated macro for GDI_CLIENT_TYPE_FROM_HANDLE (function)
macro_rules! Depcrate_ntgdiGDI_CLIENT_TYPE_FROM_HANDLE {
() => {
// Module: crate::ntgdi
// Provides: {"GDI_CLIENT_TYPE_FROM_HANDLE"}
// Dependencies: {}
# [inline] pub const fn GDI_CLIENT_TYPE_FROM_HANDLE (Handle : ULONG) -> ULONG { Handle & (GDI_HANDLE_ALTTYPE_MASK << GDI_HANDLE_ALTTYPE_SHIFT | GDI_HANDLE_TYPE_MASK << GDI_HANDLE_TYPE_SHIFT) }
};
}
