macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < T , S , const N : usize > Extend < T > for IndexSet < T , S , N > where T : Eq + Hash , S : BuildHasher , { fn extend < I > (& mut self , iterable : I) where I : IntoIterator < Item = T > , { self . map . extend (iterable . into_iter () . map (| k | (k , ()))) ; } }
    };
}

impl_143!();