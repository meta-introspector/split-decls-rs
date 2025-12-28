macro_rules! deps {
    () => {
        SelectAll!();
    };
}

macro_rules! impl_889 {
    () => {
        deps!();
        impl < St : Debug > Debug for SelectAll < St > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "SelectAll {{ ... }}") } }
    };
}

impl_889!();