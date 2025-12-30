// Generated macro for vmptrst (function)
macro_rules! Depcrate_bits64_vmxvmptrst {
() => {
// Module: crate::bits64::vmx
// Provides: {"vmptrst"}
// Dependencies: {}
# [doc = " Return current VMCS pointer."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn vmptrst () -> Result < u64 > { let value : u64 = 0 ; asm ! ("vmptrst ({0})" , in (reg) & value , options (att_syntax)) ; vmx_capture_status () . and (Ok (value)) }
};
}
