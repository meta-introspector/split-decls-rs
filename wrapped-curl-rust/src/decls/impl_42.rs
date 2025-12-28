macro_rules! deps {
    () => {
        Part!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < 'form , 'data > fmt :: Debug for Part < 'form , 'data > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Part") . field ("name" , & self . name) . field ("form" , & self . form) . finish () } }
    };
}

impl_42!();