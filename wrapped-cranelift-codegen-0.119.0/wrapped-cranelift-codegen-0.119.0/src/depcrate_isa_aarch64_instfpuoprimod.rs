// Generated macro for FPUOpRIMod (enum)
macro_rules! Depcrate_isa_aarch64_instFPUOpRIMod {
() => {
// Module: crate::isa::aarch64::inst
// Provides: {"FPUOpRIMod"}
// Dependencies: {}
# [doc = " A floating-point unit (FPU) operation with two args, a register and"] # [doc = " an immediate that modifies its dest (so takes that input value as a"] # [doc = " separate virtual register)."] # [derive (Copy , Clone , Debug)] pub enum FPUOpRIMod { # [doc = " Shift left and insert. Rd |= Rn << #imm"] Sli32 (FPULeftShiftImm) , # [doc = " Shift left and insert. Rd |= Rn << #imm"] Sli64 (FPULeftShiftImm) , }
};
}
