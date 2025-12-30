// Generated macro for impl_264 (impl)
macro_rules! Depcrate_selectimpl_264 {
() => {
// Module: crate::select
// Provides: {"impl_264"}
// Dependencies: {}
impl < T : SelectHandle > SelectHandle for & T { fn try_select (& self , token : & mut Token) -> bool { (* * self) . try_select (token) } fn deadline (& self) -> Option < Instant > { (* * self) . deadline () } fn register (& self , oper : Operation , cx : & Context) -> bool { (* * self) . register (oper , cx) } fn unregister (& self , oper : Operation) { (* * self) . unregister (oper) ; } fn accept (& self , token : & mut Token , cx : & Context) -> bool { (* * self) . accept (token , cx) } fn is_ready (& self) -> bool { (* * self) . is_ready () } fn watch (& self , oper : Operation , cx : & Context) -> bool { (* * self) . watch (oper , cx) } fn unwatch (& self , oper : Operation) { (* * self) . unwatch (oper) } }
};
}
