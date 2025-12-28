macro_rules! deps {
    () => {
        Bitness!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl Display for Bitness { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { match * self { Bitness :: Unknown => write ! (f , "unknown bitness") , Bitness :: X32 => write ! (f , "32-bit") , Bitness :: X64 => write ! (f , "64-bit") , } } }
    };
}

impl_19!()