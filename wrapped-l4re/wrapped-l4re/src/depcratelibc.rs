// Generated macro for libc (module)
macro_rules! Depcratelibc {
() => {
// Module: crate
// Provides: {"libc"}
// Dependencies: {}
mod libc { use core :: { ffi :: * , fmt } ; pub const AT_BASE : c_ulong = 7 ; pub const AT_HWCAP : c_ulong = 16 ; pub const AT_HWCAP2 : c_ulong = 26 ; # [link (name = "c")] # [link (name = "dl")] extern "C" { } extern "C" { pub fn abort () -> ! ; pub fn exit (status : i32) -> ! ; pub fn printf (format : * const c_char , ...) -> c_int ; pub fn getauxval (type_ : c_ulong) -> c_ulong ; } pub struct Stdout ; impl fmt :: Write for Stdout { fn write_str (& mut self , s : & str) -> fmt :: Result { unsafe { printf (b"%.*s\0" . as_ptr () as _ , s . len () as i32 , s . as_ptr ()) } ; Ok (()) } } }
};
}
