// Generated macro for impl_82 (impl)
macro_rules! Depcrate_fs_feature_xattrimpl_82 {
() => {
// Module: crate::fs::feature::xattr
// Provides: {"impl_82"}
// Dependencies: {}
# [cfg (not (any (target_os = "macos" , target_os = "linux" , target_os = "netbsd" , target_os = "freebsd")))] impl FileAttributes for Path { fn attributes (& self) -> io :: Result < Vec < Attribute > > { Ok (Vec :: new ()) } fn symlink_attributes (& self) -> io :: Result < Vec < Attribute > > { Ok (Vec :: new ()) } }
};
}
