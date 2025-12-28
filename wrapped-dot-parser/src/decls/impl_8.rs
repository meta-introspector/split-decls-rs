macro_rules! deps {
    () => {
        ParseError!();
        GraphFromFileError!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Display for GraphFromFileError < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { match self { Self :: FileError (e) => write ! (f , "{}" , e) , Self :: PestParseError (e) => write ! (f , "{}" , e) , Self :: ParseError (e) => write ! (f , "{}" , e) , } } }
    };
}

impl_8!()