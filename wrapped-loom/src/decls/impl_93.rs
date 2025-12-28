macro_rules! deps {
    () => {
        Ref!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Ref < T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use std :: any :: type_name ; write ! (fmt , "Ref<{}>({})" , type_name ::< T > () , self . index) } }
    };
}

impl_93!();