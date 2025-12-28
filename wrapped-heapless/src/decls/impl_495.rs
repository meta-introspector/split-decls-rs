macro_rules! deps {
    () => {
        CapacityError!();
    };
}

macro_rules! impl_495 {
    () => {
        deps!();
        impl core :: fmt :: Display for CapacityError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . write_str ("insufficient capacity") } }
    };
}

impl_495!()