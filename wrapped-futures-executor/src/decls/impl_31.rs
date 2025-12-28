macro_rules! deps {
    () => {
        Message!();
        PoolState!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl PoolState { fn send (& self , msg : Message) { self . tx . lock () . unwrap () . send (msg) . unwrap () ; } fn work (& self , idx : usize , after_start : Option < Arc < dyn Fn (usize) + Send + Sync > > , before_stop : Option < Arc < dyn Fn (usize) + Send + Sync > > ,) { let _scope = enter () . unwrap () ; if let Some (after_start) = after_start { after_start (idx) ; } loop { let msg = self . rx . lock () . unwrap () . recv () . unwrap () ; match msg { Message :: Run (task) => task . run () , Message :: Close => break , } } if let Some (before_stop) = before_stop { before_stop (idx) ; } } }
    };
}

impl_31!()