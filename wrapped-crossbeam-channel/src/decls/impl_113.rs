macro_rules! deps {
    () => {
        TryRecvError!();
        SelectHandle!();
        Channel!();
        Operation!();
        Context!();
        Token!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl SelectHandle for Channel { # [inline] fn try_select (& self , token : & mut Token) -> bool { match self . try_recv () { Ok (msg) => { token . at = Some (msg) ; true } Err (TryRecvError :: Disconnected) => { token . at = None ; true } Err (TryRecvError :: Empty) => false , } } # [inline] fn deadline (& self) -> Option < Instant > { if self . received . load (Ordering :: Relaxed) { None } else { Some (self . delivery_time) } } # [inline] fn register (& self , _oper : Operation , _cx : & Context) -> bool { self . is_ready () } # [inline] fn unregister (& self , _oper : Operation) { } # [inline] fn accept (& self , token : & mut Token , _cx : & Context) -> bool { self . try_select (token) } # [inline] fn is_ready (& self) -> bool { ! self . is_empty () } # [inline] fn watch (& self , _oper : Operation , _cx : & Context) -> bool { self . is_ready () } # [inline] fn unwatch (& self , _oper : Operation) { } }
    };
}

impl_113!();