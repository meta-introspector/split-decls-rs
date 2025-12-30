// Generated macro for impl_148 (impl)
macro_rules! Depcrate_flavors_arrayimpl_148 {
() => {
// Module: crate::flavors::array
// Provides: {"impl_148"}
// Dependencies: {}
impl < T > SelectHandle for Sender < '_ , T > { fn try_select (& self , token : & mut Token) -> bool { self . 0 . start_send (token) } fn deadline (& self) -> Option < Instant > { None } fn register (& self , oper : Operation , cx : & Context) -> bool { self . 0 . senders . register (oper , cx) ; self . is_ready () } fn unregister (& self , oper : Operation) { self . 0 . senders . unregister (oper) ; } fn accept (& self , token : & mut Token , _cx : & Context) -> bool { self . try_select (token) } fn is_ready (& self) -> bool { ! self . 0 . is_full () || self . 0 . is_disconnected () } fn watch (& self , oper : Operation , cx : & Context) -> bool { self . 0 . senders . watch (oper , cx) ; self . is_ready () } fn unwatch (& self , oper : Operation) { self . 0 . senders . unwatch (oper) ; } }
};
}
