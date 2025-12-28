macro_rules! deps {
    () => {
        DiffStats!();
        Error!();
    };
}

macro_rules! impl_353 {
    () => {
        deps!();
        impl std :: fmt :: Debug for DiffStats { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . debug_struct ("DiffStats") . field ("files_changed" , & self . files_changed ()) . field ("insertions" , & self . insertions ()) . field ("deletions" , & self . deletions ()) . finish () } }
    };
}

impl_353!();