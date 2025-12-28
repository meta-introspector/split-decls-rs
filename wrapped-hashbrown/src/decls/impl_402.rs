macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! impl_402 {
    () => {
        deps!();
        impl < T , S , A > Extend < T > for HashSet < T , S , A > where T : Eq + Hash , S : BuildHasher , A : Allocator , { # [cfg_attr (feature = "inline-more" , inline)] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { self . map . extend (iter . into_iter () . map (| k | (k , ()))) ; } # [inline] # [cfg (feature = "nightly")] fn extend_one (& mut self , k : T) { self . map . insert (k , ()) ; } # [inline] # [cfg (feature = "nightly")] fn extend_reserve (& mut self , additional : usize) { Extend :: < (T , ()) > :: extend_reserve (& mut self . map , additional) ; } }
    };
}

impl_402!()