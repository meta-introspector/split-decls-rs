macro_rules! deps {
    () => {
        Idx!();
        IndexSlice!();
        IndexVec!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < I : Idx , T > Borrow < IndexSlice < I , T > > for IndexVec < I , T > { fn borrow (& self) -> & IndexSlice < I , T > { self } }
    };
}

impl_113!()