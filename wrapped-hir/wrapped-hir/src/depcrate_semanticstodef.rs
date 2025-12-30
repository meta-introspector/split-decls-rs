// Generated macro for ToDef (trait)
macro_rules! Depcrate_semanticsToDef {
() => {
// Module: crate::semantics
// Provides: {"ToDef"}
// Dependencies: {}
pub trait ToDef : AstNode + Clone { type Def ; fn to_def (sema : & SemanticsImpl < '_ > , src : InFile < & Self >) -> Option < Self :: Def > ; }
};
}
