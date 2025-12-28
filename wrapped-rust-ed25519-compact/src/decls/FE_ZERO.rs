macro_rules! deps {
    () => {
        Fe!();
    };
}

macro_rules! FE_ZERO {
    () => {
        deps!();
        pub static FE_ZERO : Fe = Fe ([0 , 0 , 0 , 0 , 0]) ;
    };
}

FE_ZERO!()