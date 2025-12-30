// Generated macro for MaybeBorrowedStmtKind (enum)
macro_rules! Depcrate_manual_clampMaybeBorrowedStmtKind {
() => {
// Module: crate::manual_clamp
// Provides: {"MaybeBorrowedStmtKind"}
// Dependencies: {}
# [doc = " Really similar to Cow, but doesn't have a `Clone` requirement."] # [derive (Debug)] enum MaybeBorrowedStmtKind < 'a > { Borrowed (& 'a StmtKind < 'a >) , Owned (StmtKind < 'a >) , }
};
}
