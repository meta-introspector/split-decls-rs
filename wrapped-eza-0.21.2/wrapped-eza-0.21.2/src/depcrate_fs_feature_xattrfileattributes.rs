// Generated macro for FileAttributes (trait)
macro_rules! Depcrate_fs_feature_xattrFileAttributes {
() => {
// Module: crate::fs::feature::xattr
// Provides: {"FileAttributes"}
// Dependencies: {}
pub trait FileAttributes { fn attributes (& self) -> io :: Result < Vec < Attribute > > ; fn symlink_attributes (& self) -> io :: Result < Vec < Attribute > > ; }
};
}
