// Generated macro for impl_7116 (impl)
macro_rules! Depcrate_methodsimpl_7116 {
() => {
// Module: crate::methods
// Provides: {"impl_7116"}
// Dependencies: {}
impl ShouldImplTraitCase { const fn new (trait_name : & 'static str , method_name : Symbol , param_count : usize , fn_header : hir :: FnHeader , self_kind : SelfKind , output_type : OutType , lint_explicit_lifetime : bool ,) -> ShouldImplTraitCase { ShouldImplTraitCase { trait_name , method_name , param_count , fn_header , self_kind , output_type , lint_explicit_lifetime , } } fn lifetime_param_cond (& self , impl_item : & hir :: ImplItem < '_ >) -> bool { self . lint_explicit_lifetime || ! impl_item . generics . params . iter () . any (| p | { matches ! (p . kind , hir :: GenericParamKind :: Lifetime { kind : hir :: LifetimeParamKind :: Explicit }) }) } }
};
}
