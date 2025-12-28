macro_rules! adler32 {
    () => {
        # [doc = " Calculates the [adler32](https://en.wikipedia.org/wiki/Adler-32) checksum"] # [doc = " of a sequence of bytes."] # [doc = ""] # [doc = " When the pointer argument is `NULL`, the initial checksum value is returned."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must guarantee that either:"] # [doc = ""] # [doc = " - `buf` is `NULL`"] # [doc = " - `buf` and `len` satisfy the requirements of [`core::slice::from_raw_parts`]"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use libz_rs_sys::adler32;"] # [doc = ""] # [doc = " unsafe {"] # [doc = "     assert_eq!(adler32(0, core::ptr::null(), 0), 1);"] # [doc = "     assert_eq!(adler32(1, core::ptr::null(), 32), 1);"] # [doc = ""] # [doc = "     let input = [1,2,3];"] # [doc = "     assert_eq!(adler32(0, input.as_ptr(), input.len() as _), 655366);"] # [doc = " }"] # [doc = " ```"] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (adler32))] pub unsafe extern "C-unwind" fn adler32 (adler : c_ulong , buf : * const Bytef , len : uInt) -> c_ulong { adler32_z (adler , buf , len as size_t) }
    };
}

adler32!()