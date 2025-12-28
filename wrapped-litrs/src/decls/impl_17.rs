macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl Literal < & str > { # [doc = " Makes a copy of the underlying buffer and returns the owned version of"] # [doc = " `Self`."] pub fn into_owned (self) -> Literal < String > { match self { Literal :: Bool (l) => Literal :: Bool (l . to_owned ()) , Literal :: Integer (l) => Literal :: Integer (l . to_owned ()) , Literal :: Float (l) => Literal :: Float (l . to_owned ()) , Literal :: Char (l) => Literal :: Char (l . to_owned ()) , Literal :: String (l) => Literal :: String (l . into_owned ()) , Literal :: Byte (l) => Literal :: Byte (l . to_owned ()) , Literal :: ByteString (l) => Literal :: ByteString (l . into_owned ()) , Literal :: CString (l) => Literal :: CString (l . into_owned ()) , } } }
    };
}

impl_17!()