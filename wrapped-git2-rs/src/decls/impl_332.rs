macro_rules! deps {
    () => {
        Error!();
        DiffDelta!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl < 'a > std :: fmt :: Debug for DiffDelta < 'a > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . debug_struct ("DiffDelta") . field ("nfiles" , & self . nfiles ()) . field ("status" , & self . status ()) . field ("old_file" , & self . old_file ()) . field ("new_file" , & self . new_file ()) . finish () } }
    };
}

impl_332!();