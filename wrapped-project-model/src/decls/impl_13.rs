macro_rules! deps {
    () => {
        ProjectManifest!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl fmt :: Display for ProjectManifest { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self . manifest_path () , f) } }
    };
}

impl_13!()