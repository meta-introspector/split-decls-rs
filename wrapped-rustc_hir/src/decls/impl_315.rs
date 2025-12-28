macro_rules! deps {
    () => {
        Safety!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl fmt :: Display for Safety { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match * self { Self :: Unsafe => "unsafe" , Self :: Safe => "safe" , }) } }
    };
}

impl_315!();