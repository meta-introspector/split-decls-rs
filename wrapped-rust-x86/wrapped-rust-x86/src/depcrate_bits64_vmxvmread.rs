// Generated macro for vmread (function)
macro_rules! Depcrate_bits64_vmxvmread {
() => {
// Module: crate::bits64::vmx
// Provides: {"vmread"}
// Dependencies: {}
# [doc = " Read a specified field from a VMCS."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn vmread (field : u32) -> Result < u64 > { let field : u64 = field . into () ; let value : u64 ; asm ! ("vmread {0}, {1}" , in (reg) field , out (reg) value , options (att_syntax)) ; vmx_capture_status () . and (Ok (value)) }
};
}
