macro_rules! deps {
    () => {
        TypeRef!();
        Result!();
    };
}

macro_rules! impl_504 {
    () => {
        deps!();
        impl Display for TypeRef { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { TypeRef :: Named (name) => write ! (f , "{}" , name) , TypeRef :: NonNull (ty) => write ! (f , "{}!" , ty) , TypeRef :: List (ty) => write ! (f , "[{}]" , ty) , } } }
    };
}

impl_504!()