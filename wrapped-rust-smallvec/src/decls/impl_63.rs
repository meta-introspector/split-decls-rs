macro_rules! deps {
    () => {
        SmallVec!();
        IntoIter!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < T : Clone , const N : usize > Clone for IntoIter < T , N > { # [inline] fn clone (& self) -> IntoIter < T , N > { SmallVec :: from (self . as_slice ()) . into_iter () } }
    };
}

impl_63!()