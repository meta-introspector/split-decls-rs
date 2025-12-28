macro_rules! deps {
    () => {
        LinkedHashSet!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < T , S > Extend < T > for LinkedHashSet < T , S > where T : Eq + Hash , S : BuildHasher , { # [inline] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { self . map . extend (iter . into_iter () . map (| k | (k , ()))) ; } }
    };
}

impl_142!()