macro_rules! deps {
    () => {
        Result!();
        Backtrace!();
    };
}

macro_rules! impl_425 {
    () => {
        deps!();
        # [cfg (not (feature = "debug"))] impl Display for Backtrace { fn fmt (& self , _ : & mut Formatter < '_ >) -> fmt :: Result { Ok (()) } }
    };
}

impl_425!();