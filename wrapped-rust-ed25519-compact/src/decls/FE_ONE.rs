macro_rules! deps {
    () => {
        Fe!();
    };
}

macro_rules! FE_ONE {
    () => {
        deps!();
        pub static FE_ONE : Fe = Fe ([1 , 0 , 0 , 0 , 0]) ;
    };
}

FE_ONE!();