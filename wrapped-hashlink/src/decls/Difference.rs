macro_rules! deps {
    () => {
        Iter!();
        LinkedHashSet!();
    };
}

macro_rules! Difference {
    () => {
        deps!();
        pub struct Difference < 'a , T , S > { iter : Iter < 'a , T > , other : & 'a LinkedHashSet < T , S > , }
    };
}

Difference!();