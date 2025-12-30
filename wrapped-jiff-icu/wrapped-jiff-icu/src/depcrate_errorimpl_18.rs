// Generated macro for impl_18 (impl)
macro_rules! Depcrate_errorimpl_18 {
() => {
// Module: crate::error
// Provides: {"impl_18"}
// Dependencies: {}
impl core :: fmt :: Display for Error { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { match self . kind { ErrorKind :: Adhoc (ref err) => { core :: fmt :: Display :: fmt (& err . message , f) } ErrorKind :: Jiff (ref err) => err . fmt (f) , } } }
};
}
