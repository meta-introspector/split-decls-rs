macro_rules! deps {
    () => {
        AnyWrite!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < 'a > AnyWrite for dyn fmt :: Write + 'a { type Wstr = str ; type Error = fmt :: Error ; fn write_fmt (& mut self , fmt : fmt :: Arguments) -> Result < () , Self :: Error > { fmt :: Write :: write_fmt (self , fmt) } fn write_str (& mut self , s : & Self :: Wstr) -> Result < () , Self :: Error > { fmt :: Write :: write_str (self , s) } }
    };
}

impl_53!();