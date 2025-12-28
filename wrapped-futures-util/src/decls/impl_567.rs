macro_rules! deps {
    () => {
        ReuniteError!();
    };
}

macro_rules! impl_567 {
    () => {
        deps!();
        impl < T , Item > fmt :: Debug for ReuniteError < T , Item > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("ReuniteError") . field (& "...") . finish () } }
    };
}

impl_567!()