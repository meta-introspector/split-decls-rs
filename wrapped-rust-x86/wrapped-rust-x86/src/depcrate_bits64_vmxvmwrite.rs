// Generated macro for vmwrite (function)
macro_rules! Depcrate_bits64_vmxvmwrite {
() => {
// Module: crate::bits64::vmx
// Provides: {"vmwrite"}
// Dependencies: {}
# [doc = " Write to a specified field in a VMCS."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn vmwrite (field : u32 , value : u64) -> Result < () > { let field : u64 = field . into () ; asm ! ("vmwrite {1}, {0}" , in (reg) field , in (reg) value , options (att_syntax)) ; vmx_capture_status () }
};
}
