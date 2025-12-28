macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < T , S1 , S2 , const N1 : usize , const N2 : usize > PartialEq < IndexSet < T , S2 , N2 > > for IndexSet < T , S1 , N1 > where T : Eq + Hash , S1 : BuildHasher , S2 : BuildHasher , { fn eq (& self , other : & IndexSet < T , S2 , N2 >) -> bool { self . len () == other . len () && self . is_subset (other) } }
    };
}

impl_142!();