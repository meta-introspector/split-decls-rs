// Generated macro for vmptrld (function)
macro_rules! Depcrate_bits64_vmxvmptrld {
() => {
// Module: crate::bits64::vmx
// Provides: {"vmptrld"}
// Dependencies: {}
# [doc = " Load current VMCS pointer."] # [doc = ""] # [doc = " Marks the current-VMCS pointer valid and loads it with the physical address `addr`."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn vmptrld (addr : u64) -> Result < () > { asm ! ("vmptrld ({0})" , in (reg) & addr , options (att_syntax)) ; vmx_capture_status () }
};
}
