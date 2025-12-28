macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < T : Debug , const N : usize > Debug for Drain < '_ , T , N > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("Drain") . field (& self . iter . as_slice ()) . finish () } }
    };
}

impl_88!()