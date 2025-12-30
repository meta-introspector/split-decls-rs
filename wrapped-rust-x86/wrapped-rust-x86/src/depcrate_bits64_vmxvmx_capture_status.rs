// Generated macro for vmx_capture_status (function)
macro_rules! Depcrate_bits64_vmxvmx_capture_status {
() => {
// Module: crate::bits64::vmx
// Provides: {"vmx_capture_status"}
// Dependencies: {}
# [doc = " Helper used to extract VMX-specific Result in accordance with"] # [doc = " conventions described in Intel SDM, Volume 3C, Section 30.2."] # [inline (always)] fn vmx_capture_status () -> Result < () > { let flags = rflags :: read () ; if flags . contains (RFlags :: FLAGS_ZF) { Err (VmFail :: VmFailValid) } else if flags . contains (RFlags :: FLAGS_CF) { Err (VmFail :: VmFailInvalid) } else { Ok (()) } }
};
}
