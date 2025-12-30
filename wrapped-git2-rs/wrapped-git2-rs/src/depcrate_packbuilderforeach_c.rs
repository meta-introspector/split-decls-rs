// Generated macro for foreach_c (function)
macro_rules! Depcrate_packbuilderforeach_c {
() => {
// Module: crate::packbuilder
// Provides: {"foreach_c"}
// Dependencies: {}
extern "C" fn foreach_c (buf : * const c_void , size : size_t , data : * mut c_void) -> c_int { unsafe { let buf = slice :: from_raw_parts (buf as * const u8 , size as usize) ; let r = panic :: wrap (| | { let data = data as * mut & mut ForEachCb < '_ > ; (* data) (buf) }) ; if r == Some (true) { 0 } else { - 1 } } }
};
}
