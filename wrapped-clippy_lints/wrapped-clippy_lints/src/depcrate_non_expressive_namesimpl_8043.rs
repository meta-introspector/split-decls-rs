// Generated macro for impl_8043 (impl)
macro_rules! Depcrate_non_expressive_namesimpl_8043 {
() => {
// Module: crate::non_expressive_names
// Provides: {"impl_8043"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for SimilarNamesNameVisitor < '_ , 'tcx , '_ > { fn visit_pat (& mut self , pat : & 'tcx Pat) { match pat . kind { PatKind :: Ident (_ , ident , _) => { if ! pat . span . from_expansion () { self . check_ident (ident) ; } } , PatKind :: Struct (_ , _ , ref fields , _) => { for field in fields { if ! field . is_shorthand { self . visit_pat (& field . pat) ; } } } , PatKind :: Or (ref pats) => self . visit_pat (& pats [0]) , _ => walk_pat (self , pat) , } } }
};
}
