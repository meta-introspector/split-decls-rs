macro_rules! deps {
    () => {
        GenericArrayIter!();
        ArrayLength!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < T , N : ArrayLength > Drop for GenericArrayIter < T , N > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (self . as_mut_slice ()) ; } } }
    };
}

impl_51!();