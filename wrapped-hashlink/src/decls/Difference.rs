macro_rules! deps {
    () => {
        LinkedHashSet!();
        Iter!();
    };
}

macro_rules! Difference {
    () => {
        deps!();
        pub struct Difference < 'a , T , S > { iter : Iter < 'a , T > , other : & 'a LinkedHashSet < T , S > , }
    };
}

Difference!()