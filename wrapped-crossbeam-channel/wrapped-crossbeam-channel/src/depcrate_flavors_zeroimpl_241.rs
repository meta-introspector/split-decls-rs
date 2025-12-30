// Generated macro for impl_241 (impl)
macro_rules! Depcrate_flavors_zeroimpl_241 {
() => {
// Module: crate::flavors::zero
// Provides: {"impl_241"}
// Dependencies: {}
impl < T > SelectHandle for Receiver < '_ , T > { fn try_select (& self , token : & mut Token) -> bool { self . 0 . start_recv (token) } fn deadline (& self) -> Option < Instant > { None } fn register (& self , oper : Operation , cx : & Context) -> bool { let packet = Box :: into_raw (Packet :: < T > :: empty_on_heap ()) ; let mut inner = self . 0 . inner . lock () . unwrap () ; inner . receivers . register_with_packet (oper , packet . cast :: < () > () , cx) ; inner . senders . notify () ; inner . senders . can_select () || inner . is_disconnected } fn unregister (& self , oper : Operation) { if let Some (operation) = self . 0 . inner . lock () . unwrap () . receivers . unregister (oper) { unsafe { drop (Box :: from_raw (operation . packet . cast :: < Packet < T > > ())) ; } } } fn accept (& self , token : & mut Token , cx : & Context) -> bool { token . zero . 0 = cx . wait_packet () ; true } fn is_ready (& self) -> bool { let inner = self . 0 . inner . lock () . unwrap () ; inner . senders . can_select () || inner . is_disconnected } fn watch (& self , oper : Operation , cx : & Context) -> bool { let mut inner = self . 0 . inner . lock () . unwrap () ; inner . receivers . watch (oper , cx) ; inner . senders . can_select () || inner . is_disconnected } fn unwatch (& self , oper : Operation) { let mut inner = self . 0 . inner . lock () . unwrap () ; inner . receivers . unwatch (oper) ; } }
};
}
