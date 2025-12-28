macro_rules! deps {
    () => {
        SizeLimitedFmtAdapter!();
        SizeLimitExhausted!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < F : fmt :: Write > fmt :: Write for SizeLimitedFmtAdapter < F > { fn write_str (& mut self , s : & str) -> fmt :: Result { self . remaining = self . remaining . and_then (| r | r . checked_sub (s . len ()) . ok_or (SizeLimitExhausted)) ; match self . remaining { Ok (_) => self . inner . write_str (s) , Err (SizeLimitExhausted) => Err (fmt :: Error) , } } }
    };
}

impl_20!()