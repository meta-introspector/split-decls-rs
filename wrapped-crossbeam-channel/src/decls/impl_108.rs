macro_rules! deps {
    () => {
        Sender!();
        Token!();
        Operation!();
        Context!();
        SelectHandle!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < T > SelectHandle for Sender < '_ , T > { fn try_select (& self , token : & mut Token) -> bool { self . 0 . start_send (token) } fn deadline (& self) -> Option < Instant > { None } fn register (& self , oper : Operation , cx : & Context) -> bool { self . 0 . senders . register (oper , cx) ; self . is_ready () } fn unregister (& self , oper : Operation) { self . 0 . senders . unregister (oper) ; } fn accept (& self , token : & mut Token , _cx : & Context) -> bool { self . try_select (token) } fn is_ready (& self) -> bool { ! self . 0 . is_full () || self . 0 . is_disconnected () } fn watch (& self , oper : Operation , cx : & Context) -> bool { self . 0 . senders . watch (oper , cx) ; self . is_ready () } fn unwatch (& self , oper : Operation) { self . 0 . senders . unwatch (oper) ; } }
    };
}

impl_108!()