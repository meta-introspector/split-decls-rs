macro_rules! deps {
    () => {
        Idx!();
        IndexVec!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < I : Idx , T , const N : usize > From < [T ; N] > for IndexVec < I , T > { # [inline] fn from (array : [T ; N]) -> Self { IndexVec :: from_raw (array . into ()) } }
    };
}

impl_121!();