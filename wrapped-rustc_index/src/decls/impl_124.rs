macro_rules! deps {
    () => {
        Idx!();
        IndexVec!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        unsafe impl < I : Idx , T > Send for IndexVec < I , T > where T : Send { }
    };
}

impl_124!()