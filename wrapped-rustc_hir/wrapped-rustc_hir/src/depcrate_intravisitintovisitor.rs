// Generated macro for IntoVisitor (trait)
macro_rules! Depcrate_intravisitIntoVisitor {
() => {
// Module: crate::intravisit
// Provides: {"IntoVisitor"}
// Dependencies: {}
pub trait IntoVisitor < 'hir > { type Visitor : Visitor < 'hir > ; fn into_visitor (& self) -> Self :: Visitor ; }
};
}
