macro_rules! deps {
    () => {
        LinkedHashSet!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < T , S > PartialEq for LinkedHashSet < T , S > where T : Eq + Hash , S : BuildHasher , { # [inline] fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . iter () . eq (other) } }
    };
}

impl_137!()