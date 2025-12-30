// Generated macro for InputSourceInst (enum)
macro_rules! Depcrate_machinst_lowerInputSourceInst {
() => {
// Module: crate::machinst::lower
// Provides: {"InputSourceInst"}
// Dependencies: {}
# [doc = " When examining an input to an instruction, this enum provides one"] # [doc = " of several options: there is or isn't a single instruction (that"] # [doc = " we can see and merge with) that produces that input's value, and"] # [doc = " we are or aren't the single user of that instruction."] # [derive (Clone , Copy , Debug)] pub enum InputSourceInst { # [doc = " The input in question is the single, unique use of the given"] # [doc = " instruction and output index, and it can be sunk to the"] # [doc = " location of this input."] UniqueUse (Inst , usize) , # [doc = " The input in question is one of multiple uses of the given"] # [doc = " instruction. It can still be sunk to the location of this"] # [doc = " input."] Use (Inst , usize) , # [doc = " We cannot determine which instruction produced the input, or"] # [doc = " it is one of several instructions (e.g., due to a control-flow"] # [doc = " merge and blockparam), or the source instruction cannot be"] # [doc = " allowed to sink to the current location due to side-effects."] None , }
};
}
