macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < T : Debug , const N : usize > Debug for IntoIter < T , N > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("IntoIter") . field (& self . as_slice ()) . finish () } }
    };
}

impl_87!()