// Generated macro for device_id (function)
macro_rules! Depcrate_upwards_utildevice_id {
() => {
// Module: crate::upwards::util
// Provides: {"device_id"}
// Dependencies: {}
# [doc = " Returns the device ID of the directory."] # [cfg (all (unix , not (target_os = "linux")))] pub (crate) fn device_id (m : & std :: fs :: Metadata) -> u64 { use std :: os :: unix :: fs :: MetadataExt ; m . dev () }
};
}
