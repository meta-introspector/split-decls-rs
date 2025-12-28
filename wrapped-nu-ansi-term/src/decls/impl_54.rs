macro_rules! deps {
    () => {
        AnyWrite!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < 'a > AnyWrite for dyn std :: io :: Write + 'a { type Wstr = [u8] ; type Error = std :: io :: Error ; fn write_fmt (& mut self , fmt : fmt :: Arguments) -> Result < () , Self :: Error > { std :: io :: Write :: write_fmt (self , fmt) } fn write_str (& mut self , s : & Self :: Wstr) -> Result < () , Self :: Error > { std :: io :: Write :: write_all (self , s) } }
    };
}

impl_54!();