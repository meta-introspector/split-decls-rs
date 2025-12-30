// Generated macro for SyntheticAmode (enum)
macro_rules! Depcrate_isa_x64_inst_argsSyntheticAmode {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"SyntheticAmode"}
// Dependencies: {}
# [doc = " A Memory Address. These denote a 64-bit value only."] # [doc = " Used for usual addressing modes as well as addressing modes used during compilation, when the"] # [doc = " moving SP offset is not known."] # [derive (Clone , Debug)] pub enum SyntheticAmode { # [doc = " A real amode."] Real (Amode) , # [doc = " A (virtual) offset into the incoming argument area."] IncomingArg { # [doc = " The downward offset from the start of the incoming argument area."] offset : u32 , } , # [doc = " A (virtual) offset to the slot area of the function frame, which lies just above the"] # [doc = " outgoing arguments."] SlotOffset { # [doc = " The offset into the slot area."] simm32 : i32 , } , # [doc = " A virtual offset to a constant that will be emitted in the constant section of the buffer."] ConstantOffset (VCodeConstant) , }
};
}
