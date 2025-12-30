// Generated macro for LineInstruction (enum)
macro_rules! Depcrate_write_lineLineInstruction {
() => {
// Module: crate::write::line
// Provides: {"LineInstruction"}
// Dependencies: {}
# [doc = " An instruction in a line number program."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] enum LineInstruction { Special (u8) , Copy , AdvancePc (u64) , AdvanceLine (i64) , SetFile (FileId) , SetColumn (u64) , NegateStatement , SetBasicBlock , ConstAddPc , SetPrologueEnd , SetEpilogueBegin , SetIsa (u64) , EndSequence , SetAddress (Address) , SetDiscriminator (u64) , }
};
}
