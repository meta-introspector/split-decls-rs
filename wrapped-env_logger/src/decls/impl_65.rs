macro_rules! deps {
    () => {
        Formatter!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl fmt :: Debug for Formatter { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let buf = self . buf . borrow () ; f . debug_struct ("Formatter") . field ("buf" , & buf) . field ("write_style" , & self . write_style) . finish () } }
    };
}

impl_65!();