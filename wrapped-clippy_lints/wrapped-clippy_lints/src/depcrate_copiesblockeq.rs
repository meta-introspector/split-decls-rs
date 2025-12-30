// Generated macro for BlockEq (struct)
macro_rules! Depcrate_copiesBlockEq {
() => {
// Module: crate::copies
// Provides: {"BlockEq"}
// Dependencies: {}
struct BlockEq { # [doc = " The end of the range of equal stmts at the start."] start_end_eq : usize , # [doc = " The start of the range of equal stmts at the end."] end_begin_eq : Option < usize > , # [doc = " The name and id of every local which can be moved at the beginning and the end."] moved_locals : Vec < (HirId , Symbol) > , }
};
}
