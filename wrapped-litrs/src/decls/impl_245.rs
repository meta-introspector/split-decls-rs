macro_rules! deps {
    () => {
        Buffer!();
        Literal!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl < B : Buffer > fmt :: Display for Literal < B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Literal :: Bool (l) => l . fmt (f) , Literal :: Integer (l) => l . fmt (f) , Literal :: Float (l) => l . fmt (f) , Literal :: Char (l) => l . fmt (f) , Literal :: String (l) => l . fmt (f) , Literal :: Byte (l) => l . fmt (f) , Literal :: ByteString (l) => l . fmt (f) , Literal :: CString (l) => l . fmt (f) , } } }
    };
}

impl_245!()