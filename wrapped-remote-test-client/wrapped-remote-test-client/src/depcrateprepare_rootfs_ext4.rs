// Generated macro for prepare_rootfs_ext4 (function)
macro_rules! Depcrateprepare_rootfs_ext4 {
() => {
// Module: crate
// Provides: {"prepare_rootfs_ext4"}
// Dependencies: {}
fn prepare_rootfs_ext4 (rootfs : & Path , rootfs_img : & Path) { let mut dd = Command :: new ("dd") ; dd . arg ("if=/dev/zero") . arg (& format ! ("of={}" , rootfs_img . to_string_lossy ())) . arg ("bs=1M") . arg ("count=1024") ; let mut dd_child = t ! (dd . spawn ()) ; assert ! (t ! (dd_child . wait ()) . success ()) ; let mut mkfs = Command :: new ("mkfs.ext4") ; mkfs . arg ("-d") . arg (rootfs) . arg (rootfs_img) ; let mut mkfs_child = t ! (mkfs . spawn ()) ; assert ! (t ! (mkfs_child . wait ()) . success ()) ; }
};
}
