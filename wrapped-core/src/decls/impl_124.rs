macro_rules! deps {
    () => {
        ComObjectInner!();
        ComObject!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < T : ComObjectInner + core :: fmt :: Display > core :: fmt :: Display for ComObject < T > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { < T as core :: fmt :: Display > :: fmt (self . get () , f) } }
    };
}

impl_124!();