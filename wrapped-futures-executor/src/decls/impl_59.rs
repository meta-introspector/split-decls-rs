macro_rules! deps {
    () => {
        Enter!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl fmt :: Debug for Enter { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Enter") . finish () } }
    };
}

impl_59!();