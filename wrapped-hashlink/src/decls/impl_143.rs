macro_rules! deps {
    () => {
        LinkedHashSet!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < 'a , T , S > Extend < & 'a T > for LinkedHashSet < T , S > where T : 'a + Eq + Hash + Copy , S : BuildHasher , { # [inline] fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { self . extend (iter . into_iter () . cloned ()) ; } }
    };
}

impl_143!()