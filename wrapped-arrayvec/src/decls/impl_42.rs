macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < T , const CAP : usize > Deref for ArrayVec < T , CAP > { type Target = [T] ; # [inline] fn deref (& self) -> & Self :: Target { self . as_slice () } }
    };
}

impl_42!()