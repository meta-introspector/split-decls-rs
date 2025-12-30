// Generated macro for WIN32_FROM_NTSTATUS (function)
macro_rules! Depcrate_ntapi_baseWIN32_FROM_NTSTATUS {
() => {
// Module: crate::ntapi_base
// Provides: {"WIN32_FROM_NTSTATUS"}
// Dependencies: {}
# [inline] pub const fn WIN32_FROM_NTSTATUS (Status : NTSTATUS) -> ULONG { (Status as u32) & 0xffff }
};
}
