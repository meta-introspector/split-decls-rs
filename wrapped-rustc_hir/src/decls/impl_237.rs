macro_rules! deps {
    () => {
        YieldSource!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl fmt :: Display for YieldSource { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { YieldSource :: Await { .. } => "`await`" , YieldSource :: Yield => "`yield`" , }) } }
    };
}

impl_237!();