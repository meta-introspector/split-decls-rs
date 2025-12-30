// Generated macro for impl_188 (impl)
macro_rules! Depcrate_codegen_from_variant_implimpl_188 {
() => {
// Module: crate::codegen::from_variant_impl
// Provides: {"impl_188"}
// Dependencies: {}
impl ExtractAttribute for FromVariantImpl < '_ > { fn local_declarations (& self) -> TokenStream { self . base . local_declarations () } fn attr_names (& self) -> & PathList { self . attr_names } fn forward_attrs (& self) -> & ForwardAttrs < '_ > { & self . forward_attrs } fn param_name (& self) -> TokenStream { quote ! (__variant) } fn core_loop (& self) -> TokenStream { self . base . core_loop () } }
};
}
