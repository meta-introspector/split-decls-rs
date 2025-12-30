// Generated macro for CondBrTarget (enum)
macro_rules! Depcrate_isa_riscv64_instCondBrTarget {
() => {
// Module: crate::isa::riscv64::inst
// Provides: {"CondBrTarget"}
// Dependencies: {}
# [doc = " A conditional branch target."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CondBrTarget { # [doc = " An unresolved reference to a Label, as passed into"] # [doc = " `lower_branch_group()`."] Label (MachLabel) , # [doc = " No jump; fall through to the next instruction."] Fallthrough , }
};
}
