macro_rules! from_bytes_until_nul {
    () => {
        # [doc = " TODO: Change this `CStr::from_bytes_until_nul` once MSRV was bumped above 1.72.0"] fn from_bytes_until_nul (bytes : & [u8]) -> Option < & CStr > { let nul_pos = bytes . iter () . position (| & b | b == 0) ? ; Some (unsafe { CStr :: from_bytes_with_nul_unchecked (& bytes [..= nul_pos]) }) }
    };
}

from_bytes_until_nul!();