// Generated macro for MachTerminator (enum)
macro_rules! Depcrate_machinstMachTerminator {
() => {
// Module: crate::machinst
// Provides: {"MachTerminator"}
// Dependencies: {}
# [doc = " Describes a block terminator (not call) in the vcode, when its branches"] # [doc = " have not yet been finalized (so a branch may have two targets)."] # [doc = ""] # [doc = " Actual targets are not included: the single-source-of-truth for"] # [doc = " those is the VCode itself, which holds, for each block, successors"] # [doc = " and outgoing branch args per successor."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum MachTerminator { # [doc = " Not a terminator."] None , # [doc = " A return instruction."] Ret , # [doc = " A tail call."] RetCall , # [doc = " An unconditional branch to another block."] Uncond , # [doc = " A conditional branch to one of two other blocks."] Cond , # [doc = " An indirect branch with known possible targets."] Indirect , }
};
}
