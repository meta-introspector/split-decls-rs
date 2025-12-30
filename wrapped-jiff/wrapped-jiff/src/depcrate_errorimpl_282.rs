// Generated macro for impl_282 (impl)
macro_rules! Depcrate_errorimpl_282 {
() => {
// Module: crate::error
// Provides: {"impl_282"}
// Dependencies: {}
impl core :: fmt :: Display for ErrorKind { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { match * self { ErrorKind :: Adhoc (ref msg) => msg . fmt (f) , ErrorKind :: Range (ref err) => err . fmt (f) , ErrorKind :: Shared (ref err) => err . fmt (f) , ErrorKind :: FilePath (ref err) => err . fmt (f) , ErrorKind :: IO (ref err) => err . fmt (f) , } } }
};
}
