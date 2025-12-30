// Generated macro for generate (function)
macro_rules! Depcrate_property_test_codegengenerate {
() => {
// Module: crate::property_test::codegen
// Provides: {"generate"}
// Dependencies: {}
# [doc = " Generate the modified test function"] # [doc = ""] # [doc = " The rough process is:"] # [doc = "  - strip out the function args from the provided function"] # [doc = "  - turn them into a struct"] # [doc = "  - implement `Arbitrary` for that struct (simple field-wise impl)"] # [doc = "  - create a runner, do the rest"] # [doc = ""] # [doc = "  Currently, any attributes on parameters are ignored - in the future, we probably want to read"] # [doc = "  these for things like customizing strategies"] pub (super) fn generate (item_fn : ItemFn , options : Options) -> TokenStream { let (mut argless_fn , args) = strip_args (item_fn) ; let struct_tokens = generate_struct (& argless_fn . sig . ident , & args) ; let arb_tokens = arbitrary :: gen_arbitrary_impl (& argless_fn . sig . ident , & args) ; let struct_and_arb = quote ! { # struct_tokens # arb_tokens } ; let new_body = test_body :: body (* argless_fn . block , & args , struct_and_arb , & argless_fn . sig . ident , & argless_fn . sig . output , & options ,) ; * argless_fn . block = new_body ; argless_fn . attrs . push (test_attr ()) ; argless_fn . to_token_stream () }
};
}
