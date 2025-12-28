macro_rules! deps {
    () => {
        Arena!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for Arena < T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("Arena") . field ("len" , & self . len ()) . field ("data" , & self . data) . finish () } }
    };
}

impl_47!()