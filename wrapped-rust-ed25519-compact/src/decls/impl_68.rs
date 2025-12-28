macro_rules! deps {
    () => {
        Signature!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl fmt :: Debug for Signature { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_fmt (format_args ! ("{:x?}" , & self . 0)) } }
    };
}

impl_68!();