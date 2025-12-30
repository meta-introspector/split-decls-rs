// Generated macro for unix (module)
macro_rules! Depcrate_dirunix {
() => {
// Module: crate::dir
// Provides: {"unix"}
// Dependencies: {}
# [cfg (unix)] mod unix { use std :: os :: unix :: fs :: DirEntryExt ; use super :: * ; impl DirEntryExt for DirEntry { fn ino (& self) -> u64 { self . inner . ino () } } }
};
}
