macro_rules! deps {
    () => {
        Idx!();
        IndexSlice!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        unsafe impl < I : Idx , T > Send for IndexSlice < I , T > where T : Send { }
    };
}

impl_105!();