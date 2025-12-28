macro_rules! deps {
    () => {
        Idx!();
        IndexVec!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < I : Idx , T > Default for IndexVec < I , T > { # [inline] fn default () -> Self { IndexVec :: new () } }
    };
}

impl_120!()