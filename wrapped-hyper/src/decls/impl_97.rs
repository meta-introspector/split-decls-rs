macro_rules! deps {
    () => {
        Receiver!();
        Value!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl Receiver { pub (crate) fn load (& mut self , cx : & mut task :: Context < '_ >) -> Value { self . shared . waker . register (cx . waker ()) ; self . shared . value . load (Ordering :: SeqCst) } pub (crate) fn peek (& self) -> Value { self . shared . value . load (Ordering :: Relaxed) } }
    };
}

impl_97!()