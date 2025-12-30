// Generated macro for impl_195 (impl)
macro_rules! Depcrate_flavors_listimpl_195 {
() => {
// Module: crate::flavors::list
// Provides: {"impl_195"}
// Dependencies: {}
impl < T > SelectHandle for Sender < '_ , T > { fn try_select (& self , token : & mut Token) -> bool { self . 0 . start_send (token) } fn deadline (& self) -> Option < Instant > { None } fn register (& self , _oper : Operation , _cx : & Context) -> bool { self . is_ready () } fn unregister (& self , _oper : Operation) { } fn accept (& self , token : & mut Token , _cx : & Context) -> bool { self . try_select (token) } fn is_ready (& self) -> bool { true } fn watch (& self , _oper : Operation , _cx : & Context) -> bool { self . is_ready () } fn unwatch (& self , _oper : Operation) { } }
};
}
