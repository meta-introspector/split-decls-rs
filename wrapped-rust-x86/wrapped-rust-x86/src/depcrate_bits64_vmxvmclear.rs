// Generated macro for vmclear (function)
macro_rules! Depcrate_bits64_vmxvmclear {
() => {
// Module: crate::bits64::vmx
// Provides: {"vmclear"}
// Dependencies: {}
# [doc = " Clear VMCS."] # [doc = ""] # [doc = " Ensures that VMCS data maintained on the processor is copied to the VMCS region"] # [doc = " located at 4KB-aligned physical address `addr` and initializes some parts of it."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn vmclear (addr : u64) -> Result < () > { asm ! ("vmclear ({0})" , in (reg) & addr , options (att_syntax)) ; vmx_capture_status () }
};
}
