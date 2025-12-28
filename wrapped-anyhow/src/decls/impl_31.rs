macro_rules! deps {
    () => {
        Quoted!();
        Result!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl Write for Quoted < & mut fmt :: Formatter < '_ > > { fn write_str (& mut self , s : & str) -> fmt :: Result { Display :: fmt (& s . escape_debug () , self . 0) } }
    };
}

impl_31!()