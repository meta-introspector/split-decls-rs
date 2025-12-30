// Generated macro for derive (function)
macro_rules! Depcratederive {
() => {
// Module: crate
// Provides: {"derive"}
// Dependencies: {}
# [doc = " Create a builder struct for the deriving struct."] # [doc = ""] # [doc = " See the `derive_builder` crate documentation for more details."] # [proc_macro_derive (Builder , attributes (builder , builder_field_attr , builder_impl_attr , builder_setter_attr , builder_struct_attr))] pub fn derive (input : TokenStream) -> TokenStream { let ast = parse_macro_input ! (input as syn :: DeriveInput) ; derive_builder_core :: builder_for_struct (ast) . into () }
};
}
