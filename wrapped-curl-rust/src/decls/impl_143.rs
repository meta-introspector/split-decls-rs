macro_rules! deps {
    () => {
        Message!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < 'a > fmt :: Debug for Message < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Message") . field ("ptr" , & self . ptr) . finish () } }
    };
}

impl_143!();