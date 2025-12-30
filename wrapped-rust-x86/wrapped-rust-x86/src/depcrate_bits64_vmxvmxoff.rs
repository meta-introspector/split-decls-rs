// Generated macro for vmxoff (function)
macro_rules! Depcrate_bits64_vmxvmxoff {
() => {
// Module: crate::bits64::vmx
// Provides: {"vmxoff"}
// Dependencies: {}
# [doc = " Disable VMX operation."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn vmxoff () -> Result < () > { asm ! ("vmxoff") ; vmx_capture_status () }
};
}
