// Generated macro for generate_trait_impl_intransitive (function)
macro_rules! Depcrate_utilsgenerate_trait_impl_intransitive {
() => {
// Module: crate::utils
// Provides: {"generate_trait_impl_intransitive"}
// Dependencies: {}
# [doc = " Generates the corresponding `impl <trait> for Type {}` including type"] # [doc = " and lifetime parameters, with `impl`'s generic parameters' bounds kept as-is."] # [doc = ""] # [doc = " This is useful for traits like `From<T>`, since `impl<T> From<T> for U<T>` doesn't require `T: From<T>`."] pub (crate) fn generate_trait_impl_intransitive (adt : & ast :: Adt , trait_ : ast :: Type) -> ast :: Impl { generate_impl_inner (false , adt , Some (trait_) , false , None) }
};
}
