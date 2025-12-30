// Generated macro for generate_impl_with_item (function)
macro_rules! Depcrate_utilsgenerate_impl_with_item {
() => {
// Module: crate::utils
// Provides: {"generate_impl_with_item"}
// Dependencies: {}
# [doc = " Generates the corresponding `impl Type {}` including type and lifetime"] # [doc = " parameters."] pub (crate) fn generate_impl_with_item (adt : & ast :: Adt , body : Option < ast :: AssocItemList > ,) -> ast :: Impl { generate_impl_inner (false , adt , None , true , body) }
};
}
