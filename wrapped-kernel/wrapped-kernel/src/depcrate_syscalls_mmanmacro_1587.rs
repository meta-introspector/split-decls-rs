// Generated macro for macro_1587 (macro)
macro_rules! Depcrate_syscalls_mmanmacro_1587 {
() => {
// Module: crate::syscalls::mman
// Provides: {"macro_1587"}
// Dependencies: {}
bitflags ! { # [repr (transparent)] # [derive (Debug , Copy , Clone , Default)] pub struct MemoryProtection : u32 { # [doc = " Pages may not be accessed."] const None = 0 ; # [doc = " Indicates that the memory region should be readable."] const Read = 1 << 0 ; # [doc = " Indicates that the memory region should be writable."] const Write = 1 << 1 ; # [doc = " Indicates that the memory region should be executable."] const Exec = 1 << 2 ; } }
};
}
