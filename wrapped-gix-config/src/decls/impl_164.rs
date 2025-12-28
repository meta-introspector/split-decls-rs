macro_rules! deps {
    () => {
        Header!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl Display for Header < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { Display :: fmt (& self . to_bstring () , f) } }
    };
}

impl_164!()