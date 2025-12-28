macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < T , S > Extend < T > for IndexSet < T , S > where T : Hash + Eq , S : BuildHasher , { fn extend < I : IntoIterator < Item = T > > (& mut self , iterable : I) { let iter = iterable . into_iter () . map (| x | (x , ())) ; self . map . extend (iter) ; } }
    };
}

impl_85!()