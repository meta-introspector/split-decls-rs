macro_rules! deps {
    () => {
        CollectionAllocErr!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl core :: fmt :: Display for CollectionAllocErr { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "Allocation error: {:?}" , self) } }
    };
}

impl_4!()