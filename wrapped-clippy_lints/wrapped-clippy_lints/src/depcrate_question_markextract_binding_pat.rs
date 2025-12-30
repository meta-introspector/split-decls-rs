// Generated macro for extract_binding_pat (function)
macro_rules! Depcrate_question_markextract_binding_pat {
() => {
// Module: crate::question_mark
// Provides: {"extract_binding_pat"}
// Dependencies: {}
fn extract_binding_pat (pat : & Pat < '_ >) -> Option < HirId > { if let PatKind :: Binding (BindingMode :: NONE , binding , _ , None) = pat . kind { Some (binding) } else { None } }
};
}
