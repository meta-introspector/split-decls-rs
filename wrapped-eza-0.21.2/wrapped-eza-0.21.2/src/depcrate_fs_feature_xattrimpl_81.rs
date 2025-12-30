// Generated macro for impl_81 (impl)
macro_rules! Depcrate_fs_feature_xattrimpl_81 {
() => {
// Module: crate::fs::feature::xattr
// Provides: {"impl_81"}
// Dependencies: {}
# [cfg (any (target_os = "macos" , target_os = "linux" , target_os = "netbsd" , target_os = "freebsd"))] impl FileAttributes for Path { fn attributes (& self) -> io :: Result < Vec < Attribute > > { extended_attrs :: attributes (self , true) } fn symlink_attributes (& self) -> io :: Result < Vec < Attribute > > { extended_attrs :: attributes (self , false) } }
};
}
