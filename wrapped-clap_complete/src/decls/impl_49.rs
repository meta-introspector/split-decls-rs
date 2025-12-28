macro_rules! deps {
    () => {
        Shell!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl Display for Shell { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . to_possible_value () . expect ("no values are skipped") . get_name () . fmt (f) } }
    };
}

impl_49!();