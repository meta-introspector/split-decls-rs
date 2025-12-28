macro_rules! deps {
    () => {
        IndexVec!();
        Idx!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < I : Idx , T > Default for IndexVec < I , T > { # [inline] fn default () -> Self { IndexVec :: new () } }
    };
}

impl_120!();