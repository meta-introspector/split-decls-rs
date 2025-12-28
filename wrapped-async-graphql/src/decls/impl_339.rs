macro_rules! deps {
    () => {
        QueryPathSegment!();
        Result!();
        QueryPathNode!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl Display for QueryPathNode < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { let mut first = true ; self . try_for_each (| segment | { if ! first { write ! (f , ".") ? ; } first = false ; match segment { QueryPathSegment :: Index (idx) => write ! (f , "{}" , * idx) , QueryPathSegment :: Name (name) => write ! (f , "{}" , name) , } }) } }
    };
}

impl_339!()