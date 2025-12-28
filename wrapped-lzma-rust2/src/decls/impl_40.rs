macro_rules! deps {
    () => {
        Matches!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl Matches { pub (crate) fn new (count_max : usize) -> Self { Self { len : vec ! [0 ; count_max] , dist : vec ! [0 ; count_max] , count : 0 , } } }
    };
}

impl_40!()