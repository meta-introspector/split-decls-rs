macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! impl_403 {
    () => {
        deps!();
        impl < 'a , T , S , A > Extend < & 'a T > for HashSet < T , S , A > where T : 'a + Eq + Hash + Copy , S : BuildHasher , A : Allocator , { # [cfg_attr (feature = "inline-more" , inline)] fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { self . extend (iter . into_iter () . copied ()) ; } # [inline] # [cfg (feature = "nightly")] fn extend_one (& mut self , k : & 'a T) { self . map . insert (* k , ()) ; } # [inline] # [cfg (feature = "nightly")] fn extend_reserve (& mut self , additional : usize) { Extend :: < (T , ()) > :: extend_reserve (& mut self . map , additional) ; } }
    };
}

impl_403!()