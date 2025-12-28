macro_rules! deps {
    () => {
        MetaItemParser!();
    };
}

macro_rules! impl_300 {
    () => {
        deps!();
        impl < 'a > Debug for MetaItemParser < 'a > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("MetaItemParser") . field ("path" , & self . path) . field ("args" , & self . args) . finish () } }
    };
}

impl_300!();