macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < T , const CAP : usize > AsMut < [T] > for ArrayVec < T , CAP > { fn as_mut (& mut self) -> & mut [T] { self } }
    };
}

impl_80!()