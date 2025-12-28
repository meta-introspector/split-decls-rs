macro_rules! impl_389 {
    () => {
        impl fmt :: Display for NodeId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . as_u32 () , f) } }
    };
}

impl_389!()