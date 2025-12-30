// Generated macro for ctypes (module)
macro_rules! Depcratectypes {
() => {
// Module: crate
// Provides: {"ctypes"}
// Dependencies: {}
# [cfg (all (not (feature = "std") , feature = "no_std"))] pub mod ctypes { # [cfg (any (target_arch = "aarch64" , target_arch = "arm" , target_arch = "msp430" , target_arch = "powerpc" , target_arch = "powerpc64" , target_arch = "riscv32" , target_arch = "riscv64" , target_arch = "s390x" ,))] pub type c_char = c_uchar ; # [cfg (any (target_arch = "loongarch64" , target_arch = "mips" , target_arch = "mips64" , target_arch = "mips32r6" , target_arch = "mips64r6" , target_arch = "sparc" , target_arch = "sparc64" , target_arch = "x86" , target_arch = "x86_64" , target_arch = "xtensa" ,))] pub type c_char = c_schar ; pub type c_schar = i8 ; pub type c_uchar = u8 ; pub type c_short = i16 ; pub type c_ushort = u16 ; pub type c_int = i32 ; pub type c_uint = u32 ; # [cfg (target_pointer_width = "32")] pub type c_long = i32 ; # [cfg (target_pointer_width = "32")] pub type c_ulong = u32 ; # [cfg (target_pointer_width = "64")] pub type c_long = i64 ; # [cfg (target_pointer_width = "64")] pub type c_ulong = u64 ; pub type c_longlong = i64 ; pub type c_ulonglong = u64 ; pub type c_float = f32 ; pub type c_double = f64 ; pub use core :: ffi :: c_void ; }
};
}
