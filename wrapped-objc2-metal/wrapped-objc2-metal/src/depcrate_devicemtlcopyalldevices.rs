// Generated macro for MTLCopyAllDevices (function)
macro_rules! Depcrate_deviceMTLCopyAllDevices {
() => {
// Module: crate::device
// Provides: {"MTLCopyAllDevices"}
// Dependencies: {}
# [doc = " Returns all Metal devices in the system."] # [doc = ""] # [doc = " On macOS and macCatalyst, this API will not cause the system to switch"] # [doc = " devices and leaves the decision about which GPU to use up to the"] # [doc = " application based on whatever criteria it deems appropriate."] # [doc = ""] # [doc = " On iOS, tvOS and visionOS, this API returns an array containing the same"] # [doc = " device that MTLCreateSystemDefaultDevice would have returned, or an empty"] # [doc = " array if it would have failed."] # [inline] # [allow (unexpected_cfgs)] pub extern "C-unwind" fn MTLCopyAllDevices () -> Retained < NSArray < ProtocolObject < dyn MTLDevice > > > { # [cfg (any (target_os = "macos" , target_env = "macabi"))] { extern "C-unwind" { fn MTLCopyAllDevices () -> * mut NSArray < ProtocolObject < dyn MTLDevice > > ; } let ret = unsafe { MTLCopyAllDevices () } ; unsafe { Retained :: from_raw (ret) } . expect ("function was marked as returning non-null, but actually returned NULL") } # [cfg (not (any (target_os = "macos" , target_env = "macabi")))] { let device = crate :: MTLCreateSystemDefaultDevice () ; let slice : & [_] = if let Some (device) = device . as_deref () { & [device] } else { & [] } ; NSArray :: from_slice (slice) } }
};
}
