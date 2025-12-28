macro_rules! deps {
    () => {
        DequeInner!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < 'a , T : 'a + Copy , S : VecStorage < T > + ? Sized > Extend < & 'a T > for DequeInner < T , S > { fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { self . extend (iter . into_iter () . copied ()) ; } }
    };
}

impl_40!();