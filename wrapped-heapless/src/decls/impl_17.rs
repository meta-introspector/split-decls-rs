macro_rules! deps {
    () => {
        ExtendError!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl fmt :: Display for ExtendError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Capacity (error) => write ! (f , "{error}") , Self :: InteriorNul { position } => write ! (f , "interior nul byte at {position}") , } } }
    };
}

impl_17!()