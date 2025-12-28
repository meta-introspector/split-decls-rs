macro_rules! deps {
    () => {
        Split!();
    };
}

macro_rules! impl_848 {
    () => {
        deps!();
        impl < D : Debug , S > Debug for Split < D , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Split") . field ("data" , & self . data) . finish () } }
    };
}

impl_848!();