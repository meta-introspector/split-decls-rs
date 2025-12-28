macro_rules! deps {
    () => {
        ReadBuffer!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < BS : ArraySize > fmt :: Debug for ReadBuffer < BS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ReadBuffer") . field ("remaining_data" , & self . remaining ()) . finish_non_exhaustive () } }
    };
}

impl_1!();