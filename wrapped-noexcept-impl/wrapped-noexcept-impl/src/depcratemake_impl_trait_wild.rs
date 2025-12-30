// Generated macro for make_impl_trait_wild (function)
macro_rules! Depcratemake_impl_trait_wild {
() => {
// Module: crate
// Provides: {"make_impl_trait_wild"}
// Dependencies: {}
fn make_impl_trait_wild (ret : & mut Type) { match ret { # ! [cfg_attr (all (test , exhaustive) , deny (non_exhaustive_omitted_patterns))] Type :: ImplTrait (impl_trait) => { * ret = Type :: Infer (TypeInfer { underscore_token : Token ! [_] (impl_trait . impl_token . span) , }) ; } Type :: Array (ret) => make_impl_trait_wild (& mut ret . elem) , Type :: Group (ret) => make_impl_trait_wild (& mut ret . elem) , Type :: Paren (ret) => make_impl_trait_wild (& mut ret . elem) , Type :: Path (ret) => make_impl_trait_wild_in_path (& mut ret . path) , Type :: Ptr (ret) => make_impl_trait_wild (& mut ret . elem) , Type :: Reference (ret) => make_impl_trait_wild (& mut ret . elem) , Type :: Slice (ret) => make_impl_trait_wild (& mut ret . elem) , Type :: TraitObject (ret) => { for bound in & mut ret . bounds { if let TypeParamBound :: Trait (bound) = bound { make_impl_trait_wild_in_path (& mut bound . path) ; } } } Type :: Tuple (ret) => ret . elems . iter_mut () . for_each (make_impl_trait_wild) , Type :: BareFn (_) | Type :: Infer (_) | Type :: Macro (_) | Type :: Never (_) | Type :: Verbatim (_) => { } _ => { } } }
};
}
