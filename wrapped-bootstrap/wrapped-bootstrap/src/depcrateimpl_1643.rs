// Generated macro for impl_1643 (impl)
macro_rules! Depcrateimpl_1643 {
() => {
// Module: crate
// Provides: {"impl_1643"}
// Dependencies: {}
impl FileType { # [doc = " Get Unix permissions appropriate for this file type."] pub fn perms (self) -> u32 { match self { FileType :: Executable | FileType :: Script => 0o755 , FileType :: Regular | FileType :: NativeLibrary => 0o644 , } } pub fn could_have_split_debuginfo (self) -> bool { match self { FileType :: Executable | FileType :: NativeLibrary => true , FileType :: Script | FileType :: Regular => false , } } }
};
}
