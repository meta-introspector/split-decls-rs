// Generated macro for impl_13 (impl)
macro_rules! Depcrate_captureimpl_13 {
() => {
// Module: crate::capture
// Provides: {"impl_13"}
// Dependencies: {}
impl MTLCaptureDescriptor { # [doc (alias = "setCaptureObject")] # [cfg (feature = "MTLDevice")] pub fn set_capture_device (& self , device : & ProtocolObject < dyn MTLDevice >) { let device : * const _ = device ; let device : * const AnyObject = device . cast () ; unsafe { self . setCaptureObject (Some (& * device)) } } # [doc (alias = "setCaptureObject")] # [cfg (feature = "MTLCaptureScope")] pub fn set_capture_scope (& self , scope : & ProtocolObject < dyn MTLCaptureScope >) { let scope : * const _ = scope ; let scope : * const AnyObject = scope . cast () ; unsafe { self . setCaptureObject (Some (& * scope)) } } # [doc (alias = "setCaptureObject")] # [cfg (feature = "MTLCommandQueue")] pub fn set_capture_command_queue (& self , command_queue : & ProtocolObject < dyn MTLCommandQueue >) { let command_queue : * const _ = command_queue ; let command_queue : * const AnyObject = command_queue . cast () ; unsafe { self . setCaptureObject (Some (& * command_queue)) } } }
};
}
