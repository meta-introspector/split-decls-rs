macro_rules! deps {
    () => {
        OperationType!();
        Result!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl Display for OperationType { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (match self { Self :: Query => "query" , Self :: Mutation => "mutation" , Self :: Subscription => "subscription" , }) } }
    };
}

impl_38!()