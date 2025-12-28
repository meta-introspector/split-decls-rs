macro_rules! deps {
    () => {
        Patch!();
        Error!();
    };
}

macro_rules! impl_549 {
    () => {
        deps!();
        impl < 'buffers > std :: fmt :: Debug for Patch < 'buffers > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { let mut ds = f . debug_struct ("Patch") ; ds . field ("delta" , & self . delta ()) . field ("num_hunks" , & self . num_hunks ()) ; if let Ok (line_stats) = & self . line_stats () { ds . field ("line_stats" , line_stats) ; } ds . finish () } }
    };
}

impl_549!();