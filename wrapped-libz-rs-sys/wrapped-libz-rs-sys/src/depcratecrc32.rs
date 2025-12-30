// Generated macro for crc32 (function)
macro_rules! Depcratecrc32 {
() => {
// Module: crate
// Provides: {"crc32"}
// Dependencies: {}
# [doc = " Calculates the [crc32](https://en.wikipedia.org/wiki/Computation_of_cyclic_redundancy_checks#CRC-32_algorithm) checksum"] # [doc = " of a sequence of bytes."] # [doc = ""] # [doc = " When the pointer argument is `NULL`, the initial checksum value is returned."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must guarantee that either:"] # [doc = ""] # [doc = " - `buf` is `NULL`"] # [doc = " - `buf` and `len` satisfy the requirements of [`core::slice::from_raw_parts`]"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use libz_rs_sys::crc32;"] # [doc = ""] # [doc = " unsafe {"] # [doc = "     assert_eq!(crc32(0, core::ptr::null(), 0), 0);"] # [doc = "     assert_eq!(crc32(1, core::ptr::null(), 32), 0);"] # [doc = ""] # [doc = "     let input = [1,2,3];"] # [doc = "     assert_eq!(crc32(0, input.as_ptr(), input.len() as _), 1438416925);"] # [doc = " }"] # [doc = " ```"] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (crc32))] pub unsafe extern "C-unwind" fn crc32 (crc : c_ulong , buf : * const Bytef , len : uInt) -> c_ulong { crc32_z (crc , buf , len as size_t) }
};
}
