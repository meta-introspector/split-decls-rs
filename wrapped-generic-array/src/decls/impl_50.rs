macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArrayIter!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < T : fmt :: Debug , N : ArrayLength > fmt :: Debug for GenericArrayIter < T , N > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_tuple ("GenericArrayIter") . field (& self . as_slice ()) . finish () } }
    };
}

impl_50!();