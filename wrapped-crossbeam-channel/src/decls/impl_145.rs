macro_rules! deps {
    () => {
        SelectHandle!();
        Context!();
        Channel!();
        Token!();
        Operation!();
        TryRecvError!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl SelectHandle for Channel { # [inline] fn try_select (& self , token : & mut Token) -> bool { match self . try_recv () { Ok (msg) => { token . tick = Some (msg) ; true } Err (TryRecvError :: Disconnected) => { token . tick = None ; true } Err (TryRecvError :: Empty) => false , } } # [inline] fn deadline (& self) -> Option < Instant > { Some (self . delivery_time . load ()) } # [inline] fn register (& self , _oper : Operation , _cx : & Context) -> bool { self . is_ready () } # [inline] fn unregister (& self , _oper : Operation) { } # [inline] fn accept (& self , token : & mut Token , _cx : & Context) -> bool { self . try_select (token) } # [inline] fn is_ready (& self) -> bool { ! self . is_empty () } # [inline] fn watch (& self , _oper : Operation , _cx : & Context) -> bool { self . is_ready () } # [inline] fn unwatch (& self , _oper : Operation) { } }
    };
}

impl_145!()