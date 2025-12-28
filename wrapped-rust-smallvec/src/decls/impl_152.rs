macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < T , const N : usize > Borrow < [T] > for SmallVec < T , N > { # [inline] fn borrow (& self) -> & [T] { self . as_slice () } }
    };
}

impl_152!();