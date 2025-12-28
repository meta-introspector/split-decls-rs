macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl fmt :: Debug for Kind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Kind :: Label (name) => f . write_fmt (format_args ! ("Unit::Label({:?})" , name)) , Kind :: Dynamic (_) => f . write_fmt (format_args ! ("Unit::Dynamic(..)")) , } } }
    };
}

impl_129!()