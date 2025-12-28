macro_rules! deps {
    () => {
        IndexVec!();
        Idx!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        unsafe impl < I : Idx , T > Send for IndexVec < I , T > where T : Send { }
    };
}

impl_124!();