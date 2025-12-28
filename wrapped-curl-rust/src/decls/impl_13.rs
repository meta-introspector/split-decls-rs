macro_rules! deps {
    () => {
        MultiError!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl fmt :: Debug for MultiError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("MultiError") . field ("description" , & self . description ()) . field ("code" , & self . code) . finish () } }
    };
}

impl_13!()