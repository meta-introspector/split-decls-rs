macro_rules! deps {
    () => {
        VecInner!();
        LenType!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > Extend < T > for VecInner < T , LenT , S > { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = T > , { self . extend (iter) ; } }
    };
}

impl_297!();