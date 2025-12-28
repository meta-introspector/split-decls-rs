macro_rules! deps {
    () => {
        SelectHandle!();
        Operation!();
        Channel!();
        Token!();
        Context!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < T > SelectHandle for Channel < T > { # [inline] fn try_select (& self , _token : & mut Token) -> bool { false } # [inline] fn deadline (& self) -> Option < Instant > { None } # [inline] fn register (& self , _oper : Operation , _cx : & Context) -> bool { self . is_ready () } # [inline] fn unregister (& self , _oper : Operation) { } # [inline] fn accept (& self , token : & mut Token , _cx : & Context) -> bool { self . try_select (token) } # [inline] fn is_ready (& self) -> bool { false } # [inline] fn watch (& self , _oper : Operation , _cx : & Context) -> bool { self . is_ready () } # [inline] fn unwatch (& self , _oper : Operation) { } }
    };
}

impl_140!();