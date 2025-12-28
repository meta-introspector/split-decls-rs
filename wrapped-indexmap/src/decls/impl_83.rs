macro_rules! deps {
    () => {
        IndexMap!();
        IndexSet!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < T , S > FromIterator < T > for IndexSet < T , S > where T : Hash + Eq , S : BuildHasher + Default , { fn from_iter < I : IntoIterator < Item = T > > (iterable : I) -> Self { let iter = iterable . into_iter () . map (| x | (x , ())) ; IndexSet { map : IndexMap :: from_iter (iter) , } } }
    };
}

impl_83!();