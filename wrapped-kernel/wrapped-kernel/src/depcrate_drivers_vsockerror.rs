// Generated macro for error (module)
macro_rules! Depcrate_drivers_vsockerror {
() => {
// Module: crate::drivers::vsock
// Provides: {"error"}
// Dependencies: {}
# [doc = " Error module of virtio socket device driver."] pub mod error { use thiserror :: Error ; # [doc = " Virtio socket device error enum."] # [derive (Error , Debug , Copy , Clone)] pub enum VirtioVsockError { # [error ("Virtio socket device driver failed, for device {0:x}, due to a missing or malformed device config!")] NoDevCfg (u16) , # [error ("Virtio socket device driver failed, for device {0:x}, due to a missing or malformed common config!")] NoComCfg (u16) , # [error ("Virtio socket device driver failed, for device {0:x}, due to a missing or malformed ISR status config!")] NoIsrCfg (u16) , # [error ("Virtio socket device driver failed, for device {0:x}, due to a missing or malformed notification config!")] NoNotifCfg (u16) , # [error ("Virtio socket device driver failed, for device {0:x}, device did not acknowledge negotiated feature set!")] FailFeatureNeg (u16) , # [doc = " Set of features does not adhere to the requirements of features"] # [doc = " indicated by the specification"] # [error ("Virtio socket driver tried to set feature bit without setting dependency feature. Feat set: {0:?}")] FeatureRequirementsNotMet (virtio :: vsock :: F) , # [doc = " The first u64 contains the feature bits wanted by the driver."] # [doc = " but which are incompatible with the device feature set, second u64."] # [error ("Feature set: {0:?} , is incompatible with the device features: {1:?}")] IncompatibleFeatureSets (virtio :: vsock :: F , virtio :: vsock :: F) , } }
};
}
