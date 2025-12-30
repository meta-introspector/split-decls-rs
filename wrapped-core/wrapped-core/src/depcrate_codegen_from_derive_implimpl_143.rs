// Generated macro for impl_143 (impl)
macro_rules! Depcrate_codegen_from_derive_implimpl_143 {
() => {
// Module: crate::codegen::from_derive_impl
// Provides: {"impl_143"}
// Dependencies: {}
impl ExtractAttribute for FromDeriveInputImpl < '_ > { fn attr_names (& self) -> & PathList { self . attr_names } fn forward_attrs (& self) -> & ForwardAttrs < '_ > { & self . forward_attrs } fn param_name (& self) -> TokenStream { quote ! (__di) } fn core_loop (& self) -> TokenStream { self . base . core_loop () } fn local_declarations (& self) -> TokenStream { self . base . local_declarations () } }
};
}
