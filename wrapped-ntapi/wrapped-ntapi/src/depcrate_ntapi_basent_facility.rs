// Generated macro for NT_FACILITY (function)
macro_rules! Depcrate_ntapi_baseNT_FACILITY {
() => {
// Module: crate::ntapi_base
// Provides: {"NT_FACILITY"}
// Dependencies: {}
# [inline] pub const fn NT_FACILITY (Status : NTSTATUS) -> ULONG { (Status as u32) >> NT_FACILITY_SHIFT & NT_FACILITY_MASK }
};
}
