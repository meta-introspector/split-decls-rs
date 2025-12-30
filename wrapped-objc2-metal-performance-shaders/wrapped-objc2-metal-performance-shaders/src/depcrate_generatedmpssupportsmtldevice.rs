// Generated macro for MPSSupportsMTLDevice (function)
macro_rules! Depcrate_generatedMPSSupportsMTLDevice {
() => {
// Module: crate::generated
// Provides: {"MPSSupportsMTLDevice"}
// Dependencies: {}
# [doc = " MPSSupportsMTLDevice"] # [doc = ""] # [doc = " Determine whether a MetalPerformanceShaders.framework  supports a MTLDevice."] # [doc = ""] # [doc = " Use this function to determine whether a MTLDevice can be used with interfaces in MetalPerformanceShaders.framework."] # [doc = ""] # [doc = " Parameter `device`: A valid MTLDevice"] # [doc = ""] # [doc = " Returns: YES             The device is supported."] # [doc = " NO              The device is not supported"] # [inline] pub unsafe extern "C-unwind" fn MPSSupportsMTLDevice (device : Option < & ProtocolObject < dyn MTLDevice > > ,) -> bool { extern "C-unwind" { fn MPSSupportsMTLDevice (device : Option < & ProtocolObject < dyn MTLDevice > >) -> Bool ; } unsafe { MPSSupportsMTLDevice (device) } . as_bool () }
};
}
