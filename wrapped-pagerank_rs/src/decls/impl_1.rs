macro_rules! deps {
    () => {
        PagerankError!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl Display for PagerankError { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { PagerankError :: CapacityError (msg) => write ! (f , "{}" , msg) , } } }
    };
}

impl_1!()