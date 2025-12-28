macro_rules! deps {
    () => {
        IntoIter!();
        ArrayVec!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < T , const CAP : usize > Clone for IntoIter < T , CAP > where T : Clone , { fn clone (& self) -> IntoIter < T , CAP > { let mut v = ArrayVec :: new () ; v . extend_from_slice (& self . v [self . index ..]) ; v . into_iter () } }
    };
}

impl_56!();