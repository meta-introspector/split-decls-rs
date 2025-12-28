macro_rules! deps {
    () => {
        StateID!();
        SparseSet!();
    };
}

macro_rules! impl_944 {
    () => {
        deps!();
        impl core :: fmt :: Debug for SparseSet { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let elements : Vec < StateID > = self . iter () . collect () ; f . debug_tuple ("SparseSet") . field (& elements) . finish () } }
    };
}

impl_944!();