macro_rules! deps {
    () => {
        Library!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl fmt :: Debug for Library { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_fmt (format_args ! ("Library@{:p}" , self . handle)) } }
    };
}

impl_94!();