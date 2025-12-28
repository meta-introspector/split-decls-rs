macro_rules! deps {
    () => {
        ReferenceKind!();
        ResolverError!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl std :: fmt :: Display for ResolverError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: Reference (exp) => match exp { ReferenceKind :: Function { id } => write ! (f , "Unknown function: {}()" , id) , ReferenceKind :: Message { id , attribute : None , } => write ! (f , "Unknown message: {}" , id) , ReferenceKind :: Message { id , attribute : Some (attribute) , } => write ! (f , "Unknown attribute: {}.{}" , id , attribute) , ReferenceKind :: Term { id , attribute : None , } => write ! (f , "Unknown term: -{}" , id) , ReferenceKind :: Term { id , attribute : Some (attribute) , } => write ! (f , "Unknown attribute: -{}.{}" , id , attribute) , ReferenceKind :: Variable { id } => write ! (f , "Unknown variable: ${}" , id) , } , Self :: NoValue (id) => write ! (f , "No value: {}" , id) , Self :: MissingDefault => f . write_str ("No default") , Self :: Cyclic => f . write_str ("Cyclical dependency detected") , Self :: TooManyPlaceables => f . write_str ("Too many placeables") , } } }
    };
}

impl_45!();