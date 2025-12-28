macro_rules! deps {
    () => {
        FromHexError!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl fmt :: Display for FromHexError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { FromHexError :: InvalidHexCharacter { c , index } => { write ! (f , "Invalid character {c:?} at position {index}") } FromHexError :: OddLength => write ! (f , "Odd number of digits") , FromHexError :: InvalidStringLength => write ! (f , "Invalid string length") , } } }
    };
}

impl_3!()