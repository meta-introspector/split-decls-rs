macro_rules! deps {
    () => {
        Version!();
        Error!();
    };
}

macro_rules! impl_853 {
    () => {
        deps!();
        impl fmt :: Debug for Version { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { let mut f = f . debug_struct ("Version") ; f . field ("major" , & self . major) . field ("minor" , & self . minor) . field ("rev" , & self . rev) . field ("crate_version" , & self . crate_version ()) . field ("vendored" , & self . vendored ()) . field ("threads" , & self . threads ()) . field ("https" , & self . https ()) . field ("ssh" , & self . ssh ()) . field ("nsec" , & self . nsec ()) ; f . finish () } }
    };
}

impl_853!();