// Generated macro for RamFileInterface (struct)
macro_rules! Depcrate_fs_memRamFileInterface {
() => {
// Module: crate::fs::mem
// Provides: {"RamFileInterface"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct RamFileInterface { # [doc = " Position within the file"] pos : Arc < Mutex < usize > > , # [doc = " File content"] inner : Arc < RwLock < RamFileInner > > , }
};
}
