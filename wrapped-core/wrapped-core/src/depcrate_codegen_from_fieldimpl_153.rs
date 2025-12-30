// Generated macro for impl_153 (impl)
macro_rules! Depcrate_codegen_from_fieldimpl_153 {
() => {
// Module: crate::codegen::from_field
// Provides: {"impl_153"}
// Dependencies: {}
impl ExtractAttribute for FromFieldImpl < '_ > { fn attr_names (& self) -> & PathList { self . attr_names } fn forward_attrs (& self) -> & super :: ForwardAttrs < '_ > { & self . forward_attrs } fn param_name (& self) -> TokenStream { quote ! (__field) } fn core_loop (& self) -> TokenStream { self . base . core_loop () } fn local_declarations (& self) -> TokenStream { self . base . local_declarations () } }
};
}
