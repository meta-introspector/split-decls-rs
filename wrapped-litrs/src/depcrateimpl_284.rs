// Generated macro for impl_284 (impl)
macro_rules! Depcrateimpl_284 {
() => {
// Module: crate
// Provides: {"impl_284"}
// Dependencies: {}
impl < B : Buffer > fmt :: Display for Literal < B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Literal :: Bool (l) => l . fmt (f) , Literal :: Integer (l) => l . fmt (f) , Literal :: Float (l) => l . fmt (f) , Literal :: Char (l) => l . fmt (f) , Literal :: String (l) => l . fmt (f) , Literal :: Byte (l) => l . fmt (f) , Literal :: ByteString (l) => l . fmt (f) , Literal :: CString (l) => l . fmt (f) , } } }
};
}
