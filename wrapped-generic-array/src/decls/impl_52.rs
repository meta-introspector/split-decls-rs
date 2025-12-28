macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArrayIter!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < T : Clone , N : ArrayLength > Clone for GenericArrayIter < T , N > { fn clone (& self) -> Self { let mut array = unsafe { ptr :: read (& self . array) } ; let mut index_back = 0 ; for (dst , src) in array . as_mut_slice () . iter_mut () . zip (self . as_slice ()) { unsafe { ptr :: write (dst , src . clone ()) } ; index_back += 1 ; } GenericArrayIter { array , index : 0 , index_back , } } }
    };
}

impl_52!()