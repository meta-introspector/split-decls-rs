macro_rules! deps {
    () => {
        BaseType!();
        Result!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl Display for BaseType { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { Self :: Named (name) => f . write_str (name) , Self :: List (ty) => write ! (f , "[{}]" , ty) , } } }
    };
}

impl_43!()