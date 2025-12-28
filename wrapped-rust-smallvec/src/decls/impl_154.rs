macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl < T , const N : usize > AsRef < [T] > for SmallVec < T , N > { # [inline] fn as_ref (& self) -> & [T] { self . as_slice () } }
    };
}

impl_154!()