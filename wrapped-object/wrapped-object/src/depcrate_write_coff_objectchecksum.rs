// Generated macro for checksum (function)
macro_rules! Depcrate_write_coff_objectchecksum {
() => {
// Module: crate::write::coff::object
// Provides: {"checksum"}
// Dependencies: {}
fn checksum (data : & [u8]) -> u32 { let mut hasher = crc32fast :: Hasher :: new_with_initial (0xffff_ffff) ; hasher . update (data) ; ! hasher . finalize () }
};
}
