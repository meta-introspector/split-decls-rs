// Generated macro for adler32_z (function)
macro_rules! Depcrateadler32_z {
() => {
// Module: crate
// Provides: {"adler32_z"}
// Dependencies: {}
# [doc = " Calculates the [adler32](https://en.wikipedia.org/wiki/Adler-32) checksum"] # [doc = " of a sequence of bytes."] # [doc = ""] # [doc = " When the pointer argument is `NULL`, the initial checksum value is returned."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must guarantee that either:"] # [doc = ""] # [doc = " - `buf` is `NULL`"] # [doc = " - `buf` and `len` satisfy the requirements of [`core::slice::from_raw_parts`]"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use libz_rs_sys::adler32_z;"] # [doc = ""] # [doc = " unsafe {"] # [doc = "     assert_eq!(adler32_z(0, core::ptr::null(), 0), 1);"] # [doc = "     assert_eq!(adler32_z(1, core::ptr::null(), 32), 1);"] # [doc = ""] # [doc = "     let input = [1,2,3];"] # [doc = "     assert_eq!(adler32_z(0, input.as_ptr(), input.len() as _), 655366);"] # [doc = " }"] # [doc = " ```"] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (adler32_z))] pub unsafe extern "C-unwind" fn adler32_z (adler : c_ulong , buf : * const Bytef , len : size_t ,) -> c_ulong { match unsafe { slice_from_raw_parts (buf , len) } { Some (buf) => zlib_rs :: adler32 (adler as u32 , buf) as c_ulong , None => 1 , } }
};
}
