// Generated macro for impl_177 (impl)
macro_rules! Depcrate_codegen_from_type_paramimpl_177 {
() => {
// Module: crate::codegen::from_type_param
// Provides: {"impl_177"}
// Dependencies: {}
impl ExtractAttribute for FromTypeParamImpl < '_ > { fn attr_names (& self) -> & PathList { self . attr_names } fn forward_attrs (& self) -> & ForwardAttrs < '_ > { & self . forward_attrs } fn param_name (& self) -> TokenStream { quote ! (__type_param) } fn core_loop (& self) -> TokenStream { self . base . core_loop () } fn local_declarations (& self) -> TokenStream { self . base . local_declarations () } }
};
}
