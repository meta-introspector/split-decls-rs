macro_rules! deps {
    () => {
        MetaTypeName!();
        Result!();
    };
}

macro_rules! impl_1028 {
    () => {
        deps!();
        impl Display for MetaTypeName < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { MetaTypeName :: Named (name) => write ! (f , "{}" , name) , MetaTypeName :: NonNull (name) => write ! (f , "{}!" , name) , MetaTypeName :: List (name) => write ! (f , "[{}]" , name) , } } }
    };
}

impl_1028!();