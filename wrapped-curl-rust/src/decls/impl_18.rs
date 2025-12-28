macro_rules! deps {
    () => {
        FormError!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl fmt :: Debug for FormError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("FormError") . field ("description" , & self . description ()) . field ("code" , & self . code) . finish () } }
    };
}

impl_18!()