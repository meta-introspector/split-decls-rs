macro_rules! deps {
    () => {
        ManifestPath!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl fmt :: Display for ManifestPath { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . file , f) } }
    };
}

impl_76!();