// Generated macro for MiriMemoryKind (enum)
macro_rules! Depcrate_machineMiriMemoryKind {
() => {
// Module: crate::machine
// Provides: {"MiriMemoryKind"}
// Dependencies: {}
# [doc = " Extra memory kinds"] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum MiriMemoryKind { # [doc = " `__rust_alloc` memory."] Rust , # [doc = " `miri_alloc` memory."] Miri , # [doc = " `malloc` memory."] C , # [doc = " Windows `HeapAlloc` memory."] WinHeap , # [doc = " Windows \"local\" memory (to be freed with `LocalFree`)"] WinLocal , # [doc = " Memory for args, errno, env vars, and other parts of the machine-managed environment."] # [doc = " This memory may leak."] Machine , # [doc = " Memory allocated by the runtime, e.g. for readdir. Separate from `Machine` because we clean"] # [doc = " it up (or expect the user to invoke operations that clean it up) and leak-check it."] Runtime , # [doc = " Globals copied from `tcx`."] # [doc = " This memory may leak."] Global , # [doc = " Memory for extern statics."] # [doc = " This memory may leak."] ExternStatic , # [doc = " Memory for thread-local statics."] # [doc = " This memory may leak."] Tls , # [doc = " Memory mapped directly by the program"] Mmap , }
};
}
