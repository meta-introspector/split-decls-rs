macro_rules! deps {
    () => {
        ComObject!();
        ComObjectInner!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < T : ComObjectInner + core :: fmt :: Debug > core :: fmt :: Debug for ComObject < T > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { < T as core :: fmt :: Debug > :: fmt (self . get () , f) } }
    };
}

impl_123!();