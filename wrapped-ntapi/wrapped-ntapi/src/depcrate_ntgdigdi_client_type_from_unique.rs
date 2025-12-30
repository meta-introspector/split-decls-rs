// Generated macro for GDI_CLIENT_TYPE_FROM_UNIQUE (function)
macro_rules! Depcrate_ntgdiGDI_CLIENT_TYPE_FROM_UNIQUE {
() => {
// Module: crate::ntgdi
// Provides: {"GDI_CLIENT_TYPE_FROM_UNIQUE"}
// Dependencies: {}
# [inline] pub const fn GDI_CLIENT_TYPE_FROM_UNIQUE (Unique : ULONG) -> ULONG { GDI_CLIENT_TYPE_FROM_HANDLE (Unique << 16) }
};
}
