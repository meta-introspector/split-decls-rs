macro_rules! deps {
    () => {
        Event!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl Display for Event < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { Display :: fmt (& self . to_bstring () , f) } }
    };
}

impl_141!();