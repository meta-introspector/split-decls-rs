// Generated macro for vmxon (function)
macro_rules! Depcrate_bits64_vmxvmxon {
() => {
// Module: crate::bits64::vmx
// Provides: {"vmxon"}
// Dependencies: {}
# [doc = " Enable VMX operation."] # [doc = ""] # [doc = " `addr` specifies a 4KB-aligned physical address of VMXON region initialized"] # [doc = " in accordance with Intel SDM, Volume 3C, Section 24.11.5."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn vmxon (addr : u64) -> Result < () > { asm ! ("vmxon ({0})" , in (reg) & addr , options (att_syntax)) ; vmx_capture_status () }
};
}
