macro_rules! deps {
    () => {
        Tree!();
        Error!();
    };
}

macro_rules! impl_823 {
    () => {
        deps!();
        impl < 'repo > std :: fmt :: Debug for Tree < 'repo > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . debug_struct ("Tree") . field ("id" , & self . id ()) . finish () } }
    };
}

impl_823!();