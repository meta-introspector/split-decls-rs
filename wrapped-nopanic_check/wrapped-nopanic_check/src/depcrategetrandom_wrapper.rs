// Generated macro for getrandom_wrapper (function)
macro_rules! Depcrategetrandom_wrapper {
() => {
// Module: crate
// Provides: {"getrandom_wrapper"}
// Dependencies: {}
# [unsafe (no_mangle)] pub unsafe extern "C" fn getrandom_wrapper (buf_ptr : * mut u8 , buf_len : usize) -> u32 { let buf = unsafe { core :: slice :: from_raw_parts_mut (buf_ptr . cast () , buf_len) } ; let res = getrandom :: fill_uninit (buf) . map (| _ | ()) ; unsafe { core :: mem :: transmute (res) } }
};
}
