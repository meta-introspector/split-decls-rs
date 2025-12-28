macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < T : Debug , const N : usize > Debug for SmallVec < T , N > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
    };
}

impl_156!();