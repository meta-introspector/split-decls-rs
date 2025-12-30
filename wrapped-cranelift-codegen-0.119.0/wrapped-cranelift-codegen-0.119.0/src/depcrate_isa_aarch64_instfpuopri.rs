// Generated macro for FPUOpRI (enum)
macro_rules! Depcrate_isa_aarch64_instFPUOpRI {
() => {
// Module: crate::isa::aarch64::inst
// Provides: {"FPUOpRI"}
// Dependencies: {}
# [doc = " A floating-point unit (FPU) operation with two args, a register and an immediate."] # [derive (Copy , Clone , Debug)] pub enum FPUOpRI { # [doc = " Unsigned right shift. Rd = Rn << #imm"] UShr32 (FPURightShiftImm) , # [doc = " Unsigned right shift. Rd = Rn << #imm"] UShr64 (FPURightShiftImm) , }
};
}
