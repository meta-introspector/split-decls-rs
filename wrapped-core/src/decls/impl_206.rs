macro_rules! deps {
    () => {
        IUnknown!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl core :: fmt :: Debug for IUnknown { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_tuple ("IUnknown") . field (& self . as_raw ()) . finish () } }
    };
}

impl_206!()