macro_rules! deps {
    () => {
        Idx!();
        IndexVec!();
        IndexSlice!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < I : Idx , T > Deref for IndexVec < I , T > { type Target = IndexSlice < I , T > ; # [inline] fn deref (& self) -> & Self :: Target { self . as_slice () } }
    };
}

impl_111!();