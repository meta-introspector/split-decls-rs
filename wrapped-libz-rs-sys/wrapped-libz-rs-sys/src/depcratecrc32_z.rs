// Generated macro for crc32_z (function)
macro_rules! Depcratecrc32_z {
() => {
// Module: crate
// Provides: {"crc32_z"}
// Dependencies: {}
# [doc = " Calculates the [crc32](https://en.wikipedia.org/wiki/Computation_of_cyclic_redundancy_checks#CRC-32_algorithm) checksum"] # [doc = " of a sequence of bytes."] # [doc = ""] # [doc = " When the pointer argument is `NULL`, the initial checksum value is returned."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must guarantee that either:"] # [doc = ""] # [doc = " - `buf` is `NULL`"] # [doc = " - `buf` and `len` satisfy the requirements of [`core::slice::from_raw_parts`]"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use libz_rs_sys::crc32_z;"] # [doc = ""] # [doc = " unsafe {"] # [doc = "     assert_eq!(crc32_z(0, core::ptr::null(), 0), 0);"] # [doc = "     assert_eq!(crc32_z(1, core::ptr::null(), 32), 0);"] # [doc = ""] # [doc = "     let input = [1,2,3];"] # [doc = "     assert_eq!(crc32_z(0, input.as_ptr(), input.len() as _), 1438416925);"] # [doc = " }"] # [doc = " ```"] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (crc32_z))] pub unsafe extern "C-unwind" fn crc32_z (crc : c_ulong , buf : * const Bytef , len : size_t) -> c_ulong { match unsafe { slice_from_raw_parts (buf , len) } { Some (buf) => zlib_rs :: crc32 (crc as u32 , buf) as c_ulong , None => 0 , } }
};
}
