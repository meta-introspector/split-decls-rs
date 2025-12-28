macro_rules! deps {
    () => {
        BorrowCompat!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < T > core :: fmt :: Debug for BorrowCompat < T > where T : core :: fmt :: Debug , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("BorrowCompat") . field (& self . 0) . finish () } }
    };
}

impl_193!()