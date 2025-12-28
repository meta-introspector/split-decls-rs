macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < T , const CAP : usize > Drop for ArrayVec < T , CAP > { fn drop (& mut self) { self . clear () ; } }
    };
}

impl_38!()