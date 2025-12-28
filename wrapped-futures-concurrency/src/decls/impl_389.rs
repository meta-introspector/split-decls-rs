macro_rules! deps {
    () => {
        StreamGroup!();
    };
}

macro_rules! impl_389 {
    () => {
        deps!();
        impl < T : Debug > Debug for StreamGroup < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("StreamGroup") . field ("slab" , & "[..]") . finish () } }
    };
}

impl_389!();