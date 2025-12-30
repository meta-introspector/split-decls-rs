// Generated macro for vmresume (function)
macro_rules! Depcrate_bits64_vmxvmresume {
() => {
// Module: crate::bits64::vmx
// Provides: {"vmresume"}
// Dependencies: {}
# [doc = " Resume virtual machine."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] # [inline (always)] pub unsafe fn vmresume () -> Result < () > { asm ! ("vmresume") ; vmx_capture_status () }
};
}
