macro_rules! POW5 {
    () => {
        pub (crate) const POW5 : [& [u64] ; 14] = [& POW5_1 , & POW5_2 , & POW5_3 , & POW5_4 , & POW5_5 , & POW5_6 , & POW5_7 , & POW5_8 , & POW5_9 , & POW5_10 , & POW5_11 , & POW5_12 , & POW5_13 , & POW5_14 ,] ;
    };
}

POW5!();