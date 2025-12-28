macro_rules! deps {
    () => {
        ErrorKind!();
        Error!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . 0 { ErrorKind :: Char { character , index , .. } => { write ! (f , "invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-zA-Z], found `{}` at {}" , character , index) } ErrorKind :: SimpleLength { len } => { write ! (f , "invalid length: expected length 32 for simple format, found {}" , len) } ErrorKind :: GroupCount { count } => { write ! (f , "invalid group count: expected 5, found {}" , count) } ErrorKind :: GroupLength { group , len , .. } => { let expected = [8 , 4 , 4 , 4 , 12] [group] ; write ! (f , "invalid group length in group {}: expected {}, found {}" , group , expected , len) } } } }
    };
}

impl_4!();