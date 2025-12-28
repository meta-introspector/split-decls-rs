macro_rules! deps {
    () => {
        Qop!();
        QopSet!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl std :: fmt :: Debug for QopSet { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let mut l = f . debug_set () ; if (self . 0 & Qop :: Auth as u8) != 0 { l . entry (& "auth") ; } if (self . 0 & Qop :: AuthInt as u8) != 0 { l . entry (& "auth-int") ; } l . finish () } }
    };
}

impl_30!();