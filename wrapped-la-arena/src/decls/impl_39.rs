macro_rules! deps {
    () => {
        Arena!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < T > Extend < T > for Arena < T > { fn extend < II : IntoIterator < Item = T > > (& mut self , iter : II) { for t in iter { self . alloc (t) ; } } }
    };
}

impl_39!()