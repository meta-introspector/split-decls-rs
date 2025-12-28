macro_rules! impl_5 {
    () => {
        impl < F > fmt :: Debug for PollOnce < F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("PollOnce") . finish () } }
    };
}

impl_5!();