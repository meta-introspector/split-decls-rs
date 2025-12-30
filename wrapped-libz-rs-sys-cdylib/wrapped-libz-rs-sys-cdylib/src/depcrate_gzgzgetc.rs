// Generated macro for gzgetc (function)
macro_rules! Depcrate_gzgzgetc {
() => {
// Module: crate::gz
// Provides: {"gzgetc"}
// Dependencies: {}
# [doc = " Read one decompressed byte from `file`."] # [doc = ""] # [doc = " Note: The C header file zlib.h provides a macro wrapper for gzgetc that implements"] # [doc = " the fast path inline and calls this function for the slow path."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " - The byte read, on success."] # [doc = " - `-1` on error."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `file`, if non-null, must be an open file handle obtained from [`gzopen`] or [`gzdopen`]."] # [export_name = crate :: prefix ! (gzgetc)] pub unsafe extern "C-unwind" fn gzgetc (file : gzFile) -> c_int { let Some (state) = (unsafe { file . cast :: < GzState > () . as_mut () }) else { return - 1 ; } ; if state . mode != GzMode :: GZ_READ || (state . err != Z_OK && state . err != Z_BUF_ERROR) { return - 1 ; } if state . have != 0 { state . have -= 1 ; state . pos += 1 ; let ret = unsafe { * state . next } ; state . next = unsafe { state . next . add (1) } ; return c_int :: from (ret) ; } let mut c = 0u8 ; match unsafe { gz_read (state , core :: slice :: from_mut (& mut c) . as_mut_ptr () , 1) } { 1 => c_int :: from (c) , _ => - 1 , } }
};
}
