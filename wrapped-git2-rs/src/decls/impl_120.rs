macro_rules! deps {
    () => {
        Error!();
        OidArray!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < 'repo > std :: fmt :: Debug for OidArray { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . debug_tuple ("OidArray") . field (& self . deref ()) . finish () } }
    };
}

impl_120!();