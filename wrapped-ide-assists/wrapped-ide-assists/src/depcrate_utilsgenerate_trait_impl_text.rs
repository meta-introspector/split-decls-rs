// Generated macro for generate_trait_impl_text (function)
macro_rules! Depcrate_utilsgenerate_trait_impl_text {
() => {
// Module: crate::utils
// Provides: {"generate_trait_impl_text"}
// Dependencies: {}
# [doc = " Generates the surrounding `impl <trait> for Type { <code> }` including type"] # [doc = " and lifetime parameters, with `<trait>` appended to `impl`'s generic parameters' bounds."] # [doc = ""] # [doc = " This is useful for traits like `PartialEq`, since `impl<T> PartialEq for U<T>` often requires `T: PartialEq`."] # [allow (dead_code)] pub (crate) fn generate_trait_impl_text (adt : & ast :: Adt , trait_text : & str , code : & str) -> String { generate_impl_text_inner (adt , Some (trait_text) , true , code) }
};
}
