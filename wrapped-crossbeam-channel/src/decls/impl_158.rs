macro_rules! deps {
    () => {
        Sender!();
        Packet!();
        SelectHandle!();
        Context!();
        Token!();
        Operation!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < T > SelectHandle for Sender < '_ , T > { fn try_select (& self , token : & mut Token) -> bool { self . 0 . start_send (token) } fn deadline (& self) -> Option < Instant > { None } fn register (& self , oper : Operation , cx : & Context) -> bool { let packet = Box :: into_raw (Packet :: < T > :: empty_on_heap ()) ; let mut inner = self . 0 . inner . lock () . unwrap () ; inner . senders . register_with_packet (oper , packet . cast :: < () > () , cx) ; inner . receivers . notify () ; inner . receivers . can_select () || inner . is_disconnected } fn unregister (& self , oper : Operation) { if let Some (operation) = self . 0 . inner . lock () . unwrap () . senders . unregister (oper) { unsafe { drop (Box :: from_raw (operation . packet . cast :: < Packet < T > > ())) ; } } } fn accept (& self , token : & mut Token , cx : & Context) -> bool { token . zero . 0 = cx . wait_packet () ; true } fn is_ready (& self) -> bool { let inner = self . 0 . inner . lock () . unwrap () ; inner . receivers . can_select () || inner . is_disconnected } fn watch (& self , oper : Operation , cx : & Context) -> bool { let mut inner = self . 0 . inner . lock () . unwrap () ; inner . senders . watch (oper , cx) ; inner . receivers . can_select () || inner . is_disconnected } fn unwatch (& self , oper : Operation) { let mut inner = self . 0 . inner . lock () . unwrap () ; inner . senders . unwatch (oper) ; } }
    };
}

impl_158!();