macro_rules! deps {
    () => {
        Note!();
        Error!();
    };
}

macro_rules! impl_470 {
    () => {
        deps!();
        impl < 'repo > std :: fmt :: Debug for Note < 'repo > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . debug_struct ("Note") . field ("id" , & self . id ()) . finish () } }
    };
}

impl_470!()