macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Receiver < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let closed = if let Some (ref inner) = self . inner { decode_state (inner . state . load (SeqCst)) . is_closed () } else { false } ; f . debug_struct ("Receiver") . field ("closed" , & closed) . finish () } }
    };
}

impl_86!()