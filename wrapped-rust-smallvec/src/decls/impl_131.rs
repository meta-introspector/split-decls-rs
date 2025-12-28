macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < T , const N : usize > From < Vec < T > > for SmallVec < T , N > { fn from (array : Vec < T >) -> Self { Self :: from_vec (array) } }
    };
}

impl_131!();