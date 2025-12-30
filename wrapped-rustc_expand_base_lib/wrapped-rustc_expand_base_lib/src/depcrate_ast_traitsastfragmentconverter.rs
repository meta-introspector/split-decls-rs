// Generated macro for AstFragmentConverter (trait)
macro_rules! Depcrate_ast_traitsAstFragmentConverter {
() => {
// Module: crate::ast_traits
// Provides: {"AstFragmentConverter"}
// Dependencies: {}
pub trait AstFragmentConverter : Sized { type VisitOutputTy ; type FlatMapOutputTy : Default ; const KIND : AstFragmentKind ; fn to_annotatable (self) -> Annotatable ; fn fragment_to_visit_output (fragment : AstFragment) -> Self :: VisitOutputTy ; fn fragment_to_flat_map_output (fragment : AstFragment) -> Self :: FlatMapOutputTy ; }
};
}
