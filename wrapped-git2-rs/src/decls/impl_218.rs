macro_rules! deps {
    () => {
        Error!();
        Blob!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl < 'repo > std :: fmt :: Debug for Blob < 'repo > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . debug_struct ("Blob") . field ("id" , & self . id ()) . finish () } }
    };
}

impl_218!();