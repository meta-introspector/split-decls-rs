macro_rules! deps {
    () => {
        Error!();
        Tag!();
    };
}

macro_rules! impl_774 {
    () => {
        deps!();
        impl < 'repo > std :: fmt :: Debug for Tag < 'repo > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { let mut ds = f . debug_struct ("Tag") ; if let Some (name) = self . name () { ds . field ("name" , & name) ; } ds . field ("id" , & self . id ()) ; ds . finish () } }
    };
}

impl_774!()