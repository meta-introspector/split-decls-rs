macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Symbol < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . pointer { None => f . write_str ("Symbol@0x0") , Some (ptr) => f . write_fmt (format_args ! ("Symbol@{:p}" , ptr as * const ())) , } } }
    };
}

impl_121!()