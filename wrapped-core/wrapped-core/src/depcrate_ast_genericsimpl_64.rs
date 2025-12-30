// Generated macro for impl_64 (impl)
macro_rules! Depcrate_ast_genericsimpl_64 {
() => {
// Module: crate::ast::generics
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a , P : GenericParamExt > Iterator for TypeParams < 'a , P > { type Item = & 'a < P as GenericParamExt > :: TypeParam ; fn next (& mut self) -> Option < Self :: Item > { let next = self . 0 . next () ; match next { None => None , Some (v) => match v . as_type_param () { Some (val) => Some (val) , None => self . next () , } , } } }
};
}
