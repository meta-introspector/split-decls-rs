macro_rules! deps {
    () => {
        Fail!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl fmt :: Display for Fail { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Fail :: Immediately => f . write_str ("immediately") , Fail :: AfterDurationWithBackoff (duration) => { write ! (f , "after {:.02}s" , duration . as_secs_f32 ()) } } } }
    };
}

impl_2!();