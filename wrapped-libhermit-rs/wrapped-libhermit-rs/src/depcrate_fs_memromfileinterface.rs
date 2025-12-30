// Generated macro for RomFileInterface (struct)
macro_rules! Depcrate_fs_memRomFileInterface {
() => {
// Module: crate::fs::mem
// Provides: {"RomFileInterface"}
// Dependencies: {}
# [derive (Debug , Clone)] struct RomFileInterface { # [doc = " Position within the file"] pos : Arc < Mutex < usize > > , # [doc = " File content"] inner : Arc < RwLock < RomFileInner > > , }
};
}
