macro_rules! deps {
    () => {
        RegionAndOrigin!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < 'tcx > fmt :: Debug for RegionAndOrigin < 'tcx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "RegionAndOrigin({:?},{:?})" , self . region , self . origin) } }
    };
}

impl_69!();