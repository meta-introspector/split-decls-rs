macro_rules! deps {
    () => {
        Result!();
        Action!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl Action { # [doc = " Send ourselves to the given `write` which is expected to be credentials-helper compatible"] pub fn send (& self , write : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { match self { Action :: Get (ctx) => ctx . write_to (write) , Action :: Store (last) | Action :: Erase (last) => { write . write_all (last) . ok () ; write . write_all (b"\n") . ok () ; Ok (()) } } } }
    };
}

impl_18!()