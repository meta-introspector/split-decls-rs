macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < T , const N : usize > AsMut < [T] > for SmallVec < T , N > { # [inline] fn as_mut (& mut self) -> & mut [T] { self . as_mut_slice () } }
    };
}

impl_85!()