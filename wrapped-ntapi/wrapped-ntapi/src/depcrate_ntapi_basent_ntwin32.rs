// Generated macro for NT_NTWIN32 (function)
macro_rules! Depcrate_ntapi_baseNT_NTWIN32 {
() => {
// Module: crate::ntapi_base
// Provides: {"NT_NTWIN32"}
// Dependencies: {}
# [inline] pub const fn NT_NTWIN32 (Status : NTSTATUS) -> bool { NT_FACILITY (Status) == FACILITY_NTWIN32 as u32 }
};
}
