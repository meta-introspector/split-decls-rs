// Generated macro for macro_13 (macro)
macro_rules! Depcrate_capimacro_13 {
() => {
// Module: crate::capi
// Provides: {"macro_13"}
// Dependencies: {}
bitflags ! { # [doc = " Flags to either `wasmtime_mmap_{new,remap}` or `wasmtime_mprotect`."] # [repr (transparent)] # [derive (Debug , Copy , Clone , Default)] pub struct WasmProt : u32 { # [doc = " Pages may not be accessed."] const None = 0 ; # [doc = " Indicates that the memory region should be readable."] const Read = 1 << 0 ; # [doc = " Indicates that the memory region should be writable."] const Write = 1 << 1 ; # [doc = " Indicates that the memory region should be executable."] const Exec = 1 << 2 ; } }
};
}
