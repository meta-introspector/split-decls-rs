// Generated macro for error (module)
macro_rules! Depcrate_drivers_consoleerror {
() => {
// Module: crate::drivers::console
// Provides: {"error"}
// Dependencies: {}
# [doc = " Error module of virtio console device driver."] pub mod error { use thiserror :: Error ; # [doc = " Virtio console device error enum."] # [derive (Error , Debug , Copy , Clone)] pub enum VirtioConsoleError { # [cfg (feature = "pci")] # [error ("Virtio console device driver failed, for device {0:x}, due to a missing or malformed device config!")] NoDevCfg (u16) , # [doc = " The device did not acknowledge the negotiated feature set."] # [error ("Virtio console device driver failed, for device {0:x}, device did not acknowledge negotiated feature set!")] FailFeatureNeg (u16) , # [doc = " Set of features does not adhere to the requirements of features"] # [doc = " indicated by the specification"] # [error ("Virtio console driver tried to set feature bit without setting dependency feature. Feat set: {0:?}")] FeatureRequirementsNotMet (virtio :: console :: F) , # [doc = " The first u64 contains the feature bits wanted by the driver."] # [doc = " but which are incompatible with the device feature set, second u64."] # [error ("Feature set: {0:?} , is incompatible with the device features: {1:?}")] IncompatibleFeatureSets (virtio :: console :: F , virtio :: console :: F) , } }
};
}
