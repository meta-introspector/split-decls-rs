// Generated macro for VIRTIO_MAX_QUEUE_SIZE (const)
macro_rules! Depcrate_configVIRTIO_MAX_QUEUE_SIZE {
() => {
// Module: crate::config
// Provides: {"VIRTIO_MAX_QUEUE_SIZE"}
// Dependencies: {}
# [cfg (any (all (not (any (all (target_arch = "riscv64" , feature = "gem-net" , not (feature = "pci")) , feature = "rtl8139" ,)) , feature = "virtio-net" ,) , feature = "fuse" , feature = "vsock" , feature = "console" ,))] pub (crate) const VIRTIO_MAX_QUEUE_SIZE : u16 = if cfg ! (feature = "pci") { 2048 } else { 1024 } ;
};
}
