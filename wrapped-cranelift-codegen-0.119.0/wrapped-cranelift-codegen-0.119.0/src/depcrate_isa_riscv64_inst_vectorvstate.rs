// Generated macro for VState (struct)
macro_rules! Depcrate_isa_riscv64_inst_vectorVState {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"VState"}
// Dependencies: {}
# [doc = " Vector State (VState)"] # [doc = ""] # [doc = " VState represents the state of the vector unit that each instruction expects before execution."] # [doc = " Unlike VType or any of the other types here, VState is not a part of the RISC-V ISA. It is"] # [doc = " used by our instruction emission code to ensure that the vector unit is in the correct state."] # [derive (Clone , Copy , Debug , PartialEq)] pub struct VState { pub avl : VecAvl , pub vtype : VType , }
};
}
