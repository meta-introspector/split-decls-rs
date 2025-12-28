macro_rules! deps {
    () => {
        Error!();
        Commit!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl < 'repo > std :: fmt :: Debug for Commit < 'repo > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { let mut ds = f . debug_struct ("Commit") ; ds . field ("id" , & self . id ()) ; if let Some (summary) = self . summary () { ds . field ("summary" , & summary) ; } ds . finish () } }
    };
}

impl_252!()