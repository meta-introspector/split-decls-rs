// Generated macro for dirent_display (module)
macro_rules! Depcrate_syscallsdirent_display {
() => {
// Module: crate::syscalls
// Provides: {"dirent_display"}
// Dependencies: {}
mod dirent_display { use core :: ffi :: { CStr , c_char } ; use core :: fmt ; use super :: Dirent64 ; # [doc = " Helperstruct for unsafe formatting of [`Dirent64`]"] pub (super) struct Dirent64Display < 'a > { dirent : & 'a Dirent64 , } impl < 'a > Dirent64Display < 'a > { # [doc = " # Safety"] # [doc = " The `d_name` ptr of `dirent` must be valid and zero-terminated."] pub (super) unsafe fn new (dirent : & 'a Dirent64) -> Self { Self { dirent } } } impl < 'a > fmt :: Debug for Dirent64Display < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let cstr = unsafe { CStr :: from_ptr ((& raw const self . dirent . d_name) . cast :: < c_char > ()) } ; f . debug_struct ("Dirent64") . field ("d_ino" , & self . dirent . d_ino) . field ("d_off" , & self . dirent . d_off) . field ("d_reclen" , & self . dirent . d_reclen) . field ("d_type" , & self . dirent . d_type) . field ("d_name" , & cstr) . finish () } } }
};
}
