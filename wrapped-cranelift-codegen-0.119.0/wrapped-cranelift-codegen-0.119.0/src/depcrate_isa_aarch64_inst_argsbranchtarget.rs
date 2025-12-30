// Generated macro for BranchTarget (enum)
macro_rules! Depcrate_isa_aarch64_inst_argsBranchTarget {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"BranchTarget"}
// Dependencies: {}
# [doc = " A branch target. Either unresolved (basic-block index) or resolved (offset"] # [doc = " from end of current instruction)."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum BranchTarget { # [doc = " An unresolved reference to a Label, as passed into"] # [doc = " `lower_branch_group()`."] Label (MachLabel) , # [doc = " A fixed PC offset."] ResolvedOffset (i32) , }
};
}
