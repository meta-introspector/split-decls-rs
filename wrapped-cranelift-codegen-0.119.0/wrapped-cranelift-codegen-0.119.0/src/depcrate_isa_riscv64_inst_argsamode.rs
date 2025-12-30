// Generated macro for AMode (enum)
macro_rules! Depcrate_isa_riscv64_inst_argsAMode {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"AMode"}
// Dependencies: {}
# [doc = " An addressing mode specified for a load/store operation."] # [derive (Clone , Debug , Copy)] pub enum AMode { # [doc = " Arbitrary offset from a register. Converted to generation of large"] # [doc = " offsets with multiple instructions as necessary during code emission."] RegOffset (Reg , i64) , # [doc = " Offset from the stack pointer."] SPOffset (i64) , # [doc = " Offset from the frame pointer."] FPOffset (i64) , # [doc = " Offset into the slot area of the stack, which lies just above the"] # [doc = " outgoing argument area that's setup by the function prologue."] # [doc = " At emission time, this is converted to `SPOffset` with a fixup added to"] # [doc = " the offset constant. The fixup is a running value that is tracked as"] # [doc = " emission iterates through instructions in linear order, and can be"] # [doc = " adjusted up and down with [Inst::VirtualSPOffsetAdj]."] # [doc = ""] # [doc = " The standard ABI is in charge of handling this (by emitting the"] # [doc = " adjustment meta-instructions). See the diagram in the documentation"] # [doc = " for [crate::isa::aarch64::abi](the ABI module) for more details."] SlotOffset (i64) , # [doc = " Offset into the argument area."] IncomingArg (i64) , # [doc = " A reference to a constant which is placed outside of the function's"] # [doc = " body, typically at the end."] Const (VCodeConstant) , # [doc = " A reference to a label."] Label (MachLabel) , }
};
}
