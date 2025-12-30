// Generated macro for Op (enum)
macro_rules! Depcrate_opOp {
() => {
// Module: crate::op
// Provides: {"Op"}
// Dependencies: {}
# [doc = " An assignment op (e.g. `a += b`), or a binary op (e.g. `a + b`)."] # [derive (Clone , Copy , Debug , PartialEq)] enum Op { BinOp (hir :: BinOp) , AssignOp (hir :: AssignOp) , }
};
}
