macro_rules! deps {
    () => {
        IndexVec!();
        Idx!();
        IndexSlice!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < I : Idx , T > BorrowMut < IndexSlice < I , T > > for IndexVec < I , T > { fn borrow_mut (& mut self) -> & mut IndexSlice < I , T > { self } }
    };
}

impl_114!();