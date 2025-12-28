macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl :: core :: fmt :: Display for Error { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { :: core :: fmt :: Debug :: fmt (& self , f) } }
    };
}

impl_55!()