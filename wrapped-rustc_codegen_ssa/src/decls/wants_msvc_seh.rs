macro_rules! wants_msvc_seh {
    () => {
        # [doc = " Returns `true` if this session's target will use SEH-based unwinding."] # [doc = ""] # [doc = " This is only true for MSVC targets, and even then the 64-bit MSVC target"] # [doc = " currently uses SEH-ish unwinding with DWARF info tables to the side (same as"] # [doc = " 64-bit MinGW) instead of \"full SEH\"."] pub fn wants_msvc_seh (sess : & Session) -> bool { sess . target . is_like_msvc }
    };
}

wants_msvc_seh!();