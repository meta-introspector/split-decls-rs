// Generated macro for impl_133 (impl)
macro_rules! Depcrate_codegen_from_attributes_implimpl_133 {
() => {
// Module: crate::codegen::from_attributes_impl
// Provides: {"impl_133"}
// Dependencies: {}
impl ExtractAttribute for FromAttributesImpl < '_ > { fn local_declarations (& self) -> TokenStream { self . base . local_declarations () } fn attr_names (& self) -> & PathList { self . attr_names } fn forward_attrs (& self) -> & super :: ForwardAttrs < '_ > { & self . forward_attrs } fn param_name (& self) -> TokenStream { quote ! (__di) } fn attrs_accessor (& self) -> TokenStream { self . param_name () } fn core_loop (& self) -> TokenStream { self . base . core_loop () } }
};
}
