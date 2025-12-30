// Generated macro for ResolverExpand (trait)
macro_rules! Depcrate_base_expansion_contextResolverExpand {
() => {
// Module: crate::base_expansion_context
// Provides: {"ResolverExpand"}
// Dependencies: {}
pub trait ResolverExpand < DRT : OpaqueDeriveResolution + 'static > : DeriveResolutionProvider < DRT > + ImportResolver { fn next_node_id (& mut self) -> NodeId ; fn invocation_parent (& self , id : LocalExpnId) -> LocalDefId ; fn resolve_dollar_crates (& mut self) ; fn visit_ast_fragment_with_placeholders (& mut self , expn_id : LocalExpnId , fragment : & AstFragment) ; fn register_builtin_macro (& mut self , name : Symbol , ext : Arc < dyn Any + Send + Sync >) ; fn resolve_macro_invocation (& mut self , invoc : & Invocation , eager_expansion_root : LocalExpnId , force : bool ,) -> Result < Arc < dyn Any + Send + Sync > , ErrorGuaranteed > ; fn insert_impl_trait_name (& mut self , id : NodeId , name : Symbol) ; fn register_glob_delegation (& mut self , invoc_id : LocalExpnId) ; }
};
}
