macro_rules! deps {
    () => {
        Comment!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl Display for Comment < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { Display :: fmt (& self . to_bstring () , f) } }
    };
}

impl_147!();