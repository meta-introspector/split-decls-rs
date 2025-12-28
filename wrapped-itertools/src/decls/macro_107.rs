macro_rules! deps {
    () => {
        Tuple1Combination!();
    };
}

macro_rules! macro_107 {
    () => {
        deps!();
        impl_tuple_combination ! (Tuple2Combination Tuple1Combination ; a) ;
    };
}

macro_107!()