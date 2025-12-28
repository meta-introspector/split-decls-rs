macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl Display for File < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { Display :: fmt (& self . to_bstring () , f) } }
    };
}

impl_49!();