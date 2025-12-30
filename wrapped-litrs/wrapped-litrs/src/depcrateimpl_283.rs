// Generated macro for impl_283 (impl)
macro_rules! Depcrateimpl_283 {
() => {
// Module: crate
// Provides: {"impl_283"}
// Dependencies: {}
impl Literal < & str > { # [doc = " Makes a copy of the underlying buffer and returns the owned version of"] # [doc = " `Self`."] pub fn into_owned (self) -> Literal < String > { match self { Literal :: Bool (l) => Literal :: Bool (l . to_owned ()) , Literal :: Integer (l) => Literal :: Integer (l . to_owned ()) , Literal :: Float (l) => Literal :: Float (l . to_owned ()) , Literal :: Char (l) => Literal :: Char (l . to_owned ()) , Literal :: String (l) => Literal :: String (l . into_owned ()) , Literal :: Byte (l) => Literal :: Byte (l . to_owned ()) , Literal :: ByteString (l) => Literal :: ByteString (l . into_owned ()) , Literal :: CString (l) => Literal :: CString (l . into_owned ()) , } } }
};
}
