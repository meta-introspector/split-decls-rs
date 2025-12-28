macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < T , const CAP : usize > Borrow < [T] > for ArrayVec < T , CAP > { fn borrow (& self) -> & [T] { self } }
    };
}

impl_77!();