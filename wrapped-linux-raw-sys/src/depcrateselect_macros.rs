// Generated macro for select_macros (module)
macro_rules! Depcrateselect_macros {
() => {
// Module: crate
// Provides: {"select_macros"}
// Dependencies: {}
# [cfg (feature = "general")] pub mod select_macros { use crate :: ctypes :: c_int ; use crate :: general :: __kernel_fd_set ; use core :: mem :: size_of ; pub unsafe fn FD_CLR (fd : c_int , set : * mut __kernel_fd_set) { let bytes = set as * mut u8 ; if fd >= 0 { * bytes . add ((fd / 8) as usize) &= ! (1 << (fd % 8)) ; } } pub unsafe fn FD_SET (fd : c_int , set : * mut __kernel_fd_set) { let bytes = set as * mut u8 ; if fd >= 0 { * bytes . add ((fd / 8) as usize) |= 1 << (fd % 8) ; } } pub unsafe fn FD_ISSET (fd : c_int , set : * const __kernel_fd_set) -> bool { let bytes = set as * const u8 ; if fd >= 0 { * bytes . add ((fd / 8) as usize) & (1 << (fd % 8)) != 0 } else { false } } pub unsafe fn FD_ZERO (set : * mut __kernel_fd_set) { let bytes = set as * mut u8 ; core :: ptr :: write_bytes (bytes , 0 , size_of :: < __kernel_fd_set > ()) ; } }
};
}
