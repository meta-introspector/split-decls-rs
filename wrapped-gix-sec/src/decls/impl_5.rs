macro_rules! deps {
    () => {
        Mapping!();
        Trust!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < T > Mapping < T > { # [doc = " Obtain the value for the given trust `level`."] pub fn by_level (& self , level : Trust) -> & T { match level { Trust :: Full => & self . full , Trust :: Reduced => & self . reduced , } } # [doc = " Obtain the value for the given `level` once."] pub fn into_value_by_level (self , level : Trust) -> T { match level { Trust :: Full => self . full , Trust :: Reduced => self . reduced , } } }
    };
}

impl_5!()