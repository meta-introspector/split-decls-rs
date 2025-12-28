macro_rules! deps {
    () => {
        Poll!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl fmt :: Debug for Poll { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("Poll") . finish () } }
    };
}

impl_27!();