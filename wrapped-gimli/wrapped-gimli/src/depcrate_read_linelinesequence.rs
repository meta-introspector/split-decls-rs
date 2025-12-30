// Generated macro for LineSequence (struct)
macro_rules! Depcrate_read_lineLineSequence {
() => {
// Module: crate::read::line
// Provides: {"LineSequence"}
// Dependencies: {}
# [doc = " A sequence within a line number program.  A sequence, as defined in section"] # [doc = " 6.2.5 of the standard, is a linear subset of a line number program within"] # [doc = " which addresses are monotonically increasing."] # [derive (Clone , Debug)] pub struct LineSequence < R : Reader > { # [doc = " The first address that is covered by this sequence within the line number"] # [doc = " program."] pub start : u64 , # [doc = " The first address that is *not* covered by this sequence within the line"] # [doc = " number program."] pub end : u64 , instructions : LineInstructions < R > , }
};
}
