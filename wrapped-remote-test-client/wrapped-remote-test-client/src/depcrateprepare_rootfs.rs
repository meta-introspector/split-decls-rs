// Generated macro for prepare_rootfs (function)
macro_rules! Depcrateprepare_rootfs {
() => {
// Module: crate
// Provides: {"prepare_rootfs"}
// Dependencies: {}
fn prepare_rootfs (target : & str , rootfs : & Path , server : & Path , rootfs_img : & Path) { t ! (fs :: copy (server , rootfs . join ("testd"))) ; match target { "arm-unknown-linux-gnueabihf" | "aarch64-unknown-linux-gnu" => { prepare_rootfs_cpio (rootfs , rootfs_img) } "riscv64gc-unknown-linux-gnu" => prepare_rootfs_ext4 (rootfs , rootfs_img) , _ => panic ! ("{} is not supported" , target) , } }
};
}
