// Generated macro for error (module)
macro_rules! Depcrate_drivers_net_virtioerror {
() => {
// Module: crate::drivers::net::virtio
// Provides: {"error"}
// Dependencies: {}
# [doc = " Error module of virtios network driver. Containing the (VirtioNetError)[VirtioNetError]"] # [doc = " enum."] pub mod error { use thiserror :: Error ; # [doc = " Network drivers error enum."] # [derive (Error , Debug , Copy , Clone)] pub enum VirtioNetError { # [cfg (feature = "pci")] # [error ("Virtio network driver failed, for device {0:x}, due to a missing or malformed device config!")] NoDevCfg (u16) , # [error ("Virtio network driver failed, for device {0:x}, device did not acknowledge negotiated feature set!")] FailFeatureNeg (u16) , # [doc = " Set of features does not adhere to the requirements of features"] # [doc = " indicated by the specification"] # [error ("Virtio network driver tried to set feature bit without setting dependency feature. Feat set: {0:?}")] FeatureRequirementsNotMet (virtio :: net :: F) , # [doc = " The first field contains the feature bits wanted by the driver."] # [doc = " but which are incompatible with the device feature set, second field."] # [error ("Feature set: {0:?} , is incompatible with the device features: {1:?}")] IncompatibleFeatureSets (virtio :: net :: F , virtio :: net :: F) , } }
};
}
