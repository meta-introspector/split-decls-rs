macro_rules! deps {
    () => {
        Null!();
        Blob!();
        Type!();
        Result!();
    };
}

macro_rules! impl_514 {
    () => {
        deps!();
        impl fmt :: Display for Type { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Null => f . pad ("Null") , Self :: Integer => f . pad ("Integer") , Self :: Real => f . pad ("Real") , Self :: Text => f . pad ("Text") , Self :: Blob => f . pad ("Blob") , } } }
    };
}

impl_514!();