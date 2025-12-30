// Generated macro for vmlaunch (function)
macro_rules! Depcrate_bits64_vmxvmlaunch {
() => {
// Module: crate::bits64::vmx
// Provides: {"vmlaunch"}
// Dependencies: {}
# [doc = " Launch virtual machine."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] # [inline (always)] pub unsafe fn vmlaunch () -> Result < () > { asm ! ("vmlaunch") ; vmx_capture_status () }
};
}
