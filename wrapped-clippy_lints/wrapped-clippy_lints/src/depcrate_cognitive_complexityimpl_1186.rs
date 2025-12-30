// Generated macro for impl_1186 (impl)
macro_rules! Depcrate_cognitive_complexityimpl_1186 {
() => {
// Module: crate::cognitive_complexity
// Provides: {"impl_1186"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for CognitiveComplexity { fn check_fn (& mut self , cx : & LateContext < 'tcx > , kind : FnKind < 'tcx > , decl : & 'tcx FnDecl < '_ > , body : & 'tcx Body < '_ > , span : Span , def_id : LocalDefId ,) { if ! cx . tcx . has_attr (def_id , sym :: test) { let expr = if is_async_fn (kind) { match get_async_fn_body (cx . tcx , body) { Some (b) => b , None => { return ; } , } } else { body . value } ; self . check (cx , kind , decl , expr , span) ; } } fn check_attributes (& mut self , cx : & LateContext < 'tcx > , attrs : & 'tcx [Attribute]) { self . limit . push_attrs (cx . sess () , attrs , sym :: cognitive_complexity) ; } fn check_attributes_post (& mut self , cx : & LateContext < 'tcx > , attrs : & 'tcx [Attribute]) { self . limit . pop_attrs (cx . sess () , attrs , sym :: cognitive_complexity) ; } }
};
}
