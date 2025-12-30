// Generated macro for error (module)
macro_rules! Depcrate_drivers_fs_virtio_fserror {
() => {
// Module: crate::drivers::fs::virtio_fs
// Provides: {"error"}
// Dependencies: {}
# [doc = " Error module of virtios filesystem driver."] pub mod error { use thiserror :: Error ; # [doc = " Network filesystem error enum."] # [derive (Error , Debug , Copy , Clone)] pub enum VirtioFsError { # [cfg (feature = "pci")] # [error ("Virtio filesystem driver failed, for device {0:x}, due to a missing or malformed device config!")] NoDevCfg (u16) , # [error ("Virtio filesystem driver failed, for device {0:x}, device did not acknowledge negotiated feature set!")] FailFeatureNeg (u16) , # [doc = " The first field contains the feature bits wanted by the driver."] # [doc = " but which are incompatible with the device feature set, second field."] # [error ("Feature set: {0:?} , is incompatible with the device features: {1:?}")] IncompatibleFeatureSets (virtio :: fs :: F , virtio :: fs :: F) , # [doc = " Set of features does not adhere to the requirements of features"] # [doc = " indicated by the specification"] # [error ("Virtio filesystem driver tried to set feature bit without setting dependency feature. Feat set: {0:?}")] FeatureRequirementsNotMet (virtio :: fs :: F) , # [error ("Virtio filesystem failed, driver failed due unknown reason!")] Unknown , } }
};
}
