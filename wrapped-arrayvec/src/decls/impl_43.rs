macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T , const CAP : usize > DerefMut for ArrayVec < T , CAP > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { self . as_mut_slice () } }
    };
}

impl_43!();