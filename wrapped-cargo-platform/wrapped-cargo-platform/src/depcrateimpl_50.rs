// Generated macro for impl_50 (impl)
macro_rules! Depcrateimpl_50 {
() => {
// Module: crate
// Provides: {"impl_50"}
// Dependencies: {}
impl fmt :: Display for Platform { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Platform :: Name (ref n) => n . fmt (f) , Platform :: Cfg (ref e) => write ! (f , "cfg({})" , e) , } } }
};
}
