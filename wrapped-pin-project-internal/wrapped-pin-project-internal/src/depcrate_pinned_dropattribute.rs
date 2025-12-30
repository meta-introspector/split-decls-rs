// Generated macro for attribute (function)
macro_rules! Depcrate_pinned_dropattribute {
() => {
// Module: crate::pinned_drop
// Provides: {"attribute"}
// Dependencies: {}
pub (crate) fn attribute (args : & TokenStream , mut input : ItemImpl) -> TokenStream { let res = (| | -> Result < () > { if ! args . is_empty () { bail ! (args , "unexpected argument: `{}`" , args) } validate_impl (& input) ? ; expand_impl (& mut input) ; Ok (()) }) () ; if let Err (e) = res { let mut tokens = e . to_compile_error () ; if let Type :: Path (self_ty) = & * input . self_ty { let (impl_generics , _ , where_clause) = input . generics . split_for_impl () ; tokens . extend (quote ! { impl # impl_generics :: pin_project :: __private :: PinnedDrop for # self_ty # where_clause { unsafe fn drop (self : :: pin_project :: __private :: Pin <& mut Self >) { } } }) ; } tokens } else { input . into_token_stream () } }
};
}
