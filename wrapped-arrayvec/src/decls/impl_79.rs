macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < T , const CAP : usize > AsRef < [T] > for ArrayVec < T , CAP > { fn as_ref (& self) -> & [T] { self } }
    };
}

impl_79!()