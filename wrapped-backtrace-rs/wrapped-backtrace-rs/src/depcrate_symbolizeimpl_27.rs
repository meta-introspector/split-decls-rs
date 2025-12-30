// Generated macro for impl_27 (impl)
macro_rules! Depcrate_symbolizeimpl_27 {
() => {
// Module: crate::symbolize
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'a > ResolveWhat < 'a > { # [allow (dead_code)] fn address_or_ip (& self) -> * mut c_void { match self { ResolveWhat :: Address (a) => adjust_ip (* a) , ResolveWhat :: Frame (f) => adjust_ip (f . ip ()) , } } }
};
}
