macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl fmt :: Debug for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Error") . field ("description" , & self . description ()) . field ("code" , & self . code) . field ("extra" , & self . extra) . finish () } }
    };
}

impl_3!()