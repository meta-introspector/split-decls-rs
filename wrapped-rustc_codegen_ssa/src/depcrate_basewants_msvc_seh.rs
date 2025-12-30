// Generated macro for wants_msvc_seh (function)
macro_rules! Depcrate_basewants_msvc_seh {
() => {
// Module: crate::base
// Provides: {"wants_msvc_seh"}
// Dependencies: {}
# [doc = " Returns `true` if this session's target will use SEH-based unwinding."] # [doc = ""] # [doc = " This is only true for MSVC targets, and even then the 64-bit MSVC target"] # [doc = " currently uses SEH-ish unwinding with DWARF info tables to the side (same as"] # [doc = " 64-bit MinGW) instead of \"full SEH\"."] pub fn wants_msvc_seh (sess : & Session) -> bool { sess . target . is_like_msvc }
};
}
