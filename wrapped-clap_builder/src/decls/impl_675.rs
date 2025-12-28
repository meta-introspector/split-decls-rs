macro_rules! deps {
    () => {
        Result!();
        ColorChoice!();
    };
}

macro_rules! impl_675 {
    () => {
        deps!();
        impl std :: fmt :: Display for ColorChoice { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . to_possible_value () . expect ("no values are skipped") . get_name () . fmt (f) } }
    };
}

impl_675!()