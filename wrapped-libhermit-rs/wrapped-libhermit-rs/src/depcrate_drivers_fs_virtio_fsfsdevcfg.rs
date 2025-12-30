// Generated macro for FsDevCfg (struct)
macro_rules! Depcrate_drivers_fs_virtio_fsFsDevCfg {
() => {
// Module: crate::drivers::fs::virtio_fs
// Provides: {"FsDevCfg"}
// Dependencies: {}
# [doc = " A wrapper struct for the raw configuration structure."] # [doc = " Handling the right access to fields, as some are read-only"] # [doc = " for the driver."] pub (crate) struct FsDevCfg { pub raw : VolatileRef < 'static , virtio :: fs :: Config , ReadOnly > , pub dev_id : u16 , pub features : virtio :: fs :: F , }
};
}
