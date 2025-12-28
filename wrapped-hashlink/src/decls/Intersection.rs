macro_rules! deps {
    () => {
        Iter!();
        LinkedHashSet!();
    };
}

macro_rules! Intersection {
    () => {
        deps!();
        pub struct Intersection < 'a , T , S > { iter : Iter < 'a , T > , other : & 'a LinkedHashSet < T , S > , }
    };
}

Intersection!()