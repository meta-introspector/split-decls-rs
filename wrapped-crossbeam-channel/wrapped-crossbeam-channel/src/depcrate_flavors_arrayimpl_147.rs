// Generated macro for impl_147 (impl)
macro_rules! Depcrate_flavors_arrayimpl_147 {
() => {
// Module: crate::flavors::array
// Provides: {"impl_147"}
// Dependencies: {}
impl < T > SelectHandle for Receiver < '_ , T > { fn try_select (& self , token : & mut Token) -> bool { self . 0 . start_recv (token) } fn deadline (& self) -> Option < Instant > { None } fn register (& self , oper : Operation , cx : & Context) -> bool { self . 0 . receivers . register (oper , cx) ; self . is_ready () } fn unregister (& self , oper : Operation) { self . 0 . receivers . unregister (oper) ; } fn accept (& self , token : & mut Token , _cx : & Context) -> bool { self . try_select (token) } fn is_ready (& self) -> bool { ! self . 0 . is_empty () || self . 0 . is_disconnected () } fn watch (& self , oper : Operation , cx : & Context) -> bool { self . 0 . receivers . watch (oper , cx) ; self . is_ready () } fn unwatch (& self , oper : Operation) { self . 0 . receivers . unwatch (oper) ; } }
};
}
