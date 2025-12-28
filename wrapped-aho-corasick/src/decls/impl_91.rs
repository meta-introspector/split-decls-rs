macro_rules! deps {
    () => {
        Match!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Match { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "Match(pid: {:?}, link: {:?})" , self . pattern () . as_usize () , self . link () . as_usize ()) } }
    };
}

impl_91!();