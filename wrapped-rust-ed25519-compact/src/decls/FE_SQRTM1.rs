macro_rules! deps {
    () => {
        Fe!();
    };
}

macro_rules! FE_SQRTM1 {
    () => {
        deps!();
        pub static FE_SQRTM1 : Fe = Fe ([1718705420411056 , 234908883556509 , 2233514472574048 , 2117202627021982 , 765476049583133 ,]) ;
    };
}

FE_SQRTM1!();