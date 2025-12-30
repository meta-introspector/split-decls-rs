// Generated macro for impl_6505 (impl)
macro_rules! Depcrate_methods_should_implement_traitimpl_6505 {
() => {
// Module: crate::methods::should_implement_trait
// Provides: {"impl_6505"}
// Dependencies: {}
impl ShouldImplTraitCase { const fn new (trait_name : & 'static str , method_name : Symbol , param_count : usize , self_kind : SelfKind , output_type : OutType , lint_explicit_lifetime : bool , in_prelude_since : Edition ,) -> ShouldImplTraitCase { ShouldImplTraitCase { trait_name , method_name , param_count , self_kind , output_type , lint_explicit_lifetime , in_prelude_since , } } fn lifetime_param_cond (& self , impl_item : & ImplItem < '_ >) -> bool { self . lint_explicit_lifetime || ! impl_item . generics . params . iter () . any (| p | { matches ! (p . kind , GenericParamKind :: Lifetime { kind : LifetimeParamKind :: Explicit }) }) } }
};
}
