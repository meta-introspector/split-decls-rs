macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < T , S1 , S2 > PartialEq < IndexSet < T , S2 > > for IndexSet < T , S1 > where T : Hash + Eq , S1 : BuildHasher , S2 : BuildHasher , { fn eq (& self , other : & IndexSet < T , S2 >) -> bool { self . len () == other . len () && self . is_subset (other) } }
    };
}

impl_88!()