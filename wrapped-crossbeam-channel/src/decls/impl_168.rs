macro_rules! deps {
    () => {
        Context!();
        Token!();
        SelectHandle!();
        Operation!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < T : SelectHandle > SelectHandle for & T { fn try_select (& self , token : & mut Token) -> bool { (* * self) . try_select (token) } fn deadline (& self) -> Option < Instant > { (* * self) . deadline () } fn register (& self , oper : Operation , cx : & Context) -> bool { (* * self) . register (oper , cx) } fn unregister (& self , oper : Operation) { (* * self) . unregister (oper) ; } fn accept (& self , token : & mut Token , cx : & Context) -> bool { (* * self) . accept (token , cx) } fn is_ready (& self) -> bool { (* * self) . is_ready () } fn watch (& self , oper : Operation , cx : & Context) -> bool { (* * self) . watch (oper , cx) } fn unwatch (& self , oper : Operation) { (* * self) . unwatch (oper) } }
    };
}

impl_168!()