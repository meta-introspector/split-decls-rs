macro_rules! deps {
    () => {
        LenType!();
        VecInner!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl < 'a , T , LenT : LenType , S : VecStorage < T > + ? Sized > Extend < & 'a T > for VecInner < T , LenT , S > where T : 'a + Copy , { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = & 'a T > , { self . extend (iter . into_iter () . cloned ()) ; } }
    };
}

impl_298!()