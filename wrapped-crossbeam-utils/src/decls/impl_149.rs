macro_rules! deps {
    () => {
        WaitGroup!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl fmt :: Debug for WaitGroup { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let count : & usize = & self . inner . count . lock () . unwrap () ; f . debug_struct ("WaitGroup") . field ("count" , count) . finish () } }
    };
}

impl_149!()