// Generated macro for rustc_demangle (function)
macro_rules! Depcraterustc_demangle {
() => {
// Module: crate
// Provides: {"rustc_demangle"}
// Dependencies: {}
# [doc = " C-style interface for demangling."] # [doc = " Demangles symbol given in `mangled` argument into `out` buffer"] # [doc = ""] # [doc = " Unsafe as it handles buffers by raw pointers."] # [doc = ""] # [doc = " Returns 0 if `mangled` is not Rust symbol or if `out` buffer is too small"] # [doc = " Returns 1 otherwise"] # [no_mangle] pub unsafe extern "C" fn rustc_demangle (mangled : * const c_char , out : * mut c_char , out_size : usize ,) -> c_int { let mangled_str = match std :: ffi :: CStr :: from_ptr (mangled) . to_str () { Ok (s) => s , Err (_) => return 0 , } ; match rustc_demangle :: try_demangle (mangled_str) { Ok (demangle) => { let mut out_slice = std :: slice :: from_raw_parts_mut (out as * mut u8 , out_size) ; match write ! (out_slice , "{:#}\0" , demangle) { Ok (_) => return 1 , Err (_) => return 0 , } } Err (_) => return 0 , } }
};
}
