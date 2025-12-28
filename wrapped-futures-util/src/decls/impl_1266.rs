macro_rules! deps {
    () => {
        ReuniteError!();
    };
}

macro_rules! impl_1266 {
    () => {
        deps!();
        impl < T > fmt :: Debug for ReuniteError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("ReuniteError") . field (& "...") . finish () } }
    };
}

impl_1266!()