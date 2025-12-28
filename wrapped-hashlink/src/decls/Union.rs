macro_rules! deps {
    () => {
        Iter!();
        Difference!();
    };
}

macro_rules! Union {
    () => {
        deps!();
        pub struct Union < 'a , T , S > { iter : Chain < Iter < 'a , T > , Difference < 'a , T , S > > , }
    };
}

Union!()