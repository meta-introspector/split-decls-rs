// Generated macro for replace_impl_trait_with_infer (function)
macro_rules! Depcrate_expandreplace_impl_trait_with_infer {
() => {
// Module: crate::expand
// Provides: {"replace_impl_trait_with_infer"}
// Dependencies: {}
fn replace_impl_trait_with_infer (ty : & mut Type) { struct ReplaceImplTraitWithInfer ; impl VisitMut for ReplaceImplTraitWithInfer { fn visit_type_mut (& mut self , ty : & mut Type) { if let Type :: ImplTrait (impl_trait) = ty { * ty = Type :: Infer (TypeInfer { underscore_token : Token ! [_] (impl_trait . impl_token . span) , }) ; } visit_mut :: visit_type_mut (self , ty) ; } } ReplaceImplTraitWithInfer . visit_type_mut (ty) ; }
};
}
