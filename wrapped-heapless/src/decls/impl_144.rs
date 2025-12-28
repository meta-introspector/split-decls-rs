macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < 'a , T , S , const N : usize > Extend < & 'a T > for IndexSet < T , S , N > where T : 'a + Eq + Hash + Copy , S : BuildHasher , { fn extend < I > (& mut self , iterable : I) where I : IntoIterator < Item = & 'a T > , { self . extend (iterable . into_iter () . cloned ()) ; } }
    };
}

impl_144!()