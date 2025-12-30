// Generated macro for impl_64 (impl)
macro_rules! Depcrate_parseimpl_64 {
() => {
// Module: crate::parse
// Provides: {"impl_64"}
// Dependencies: {}
impl fmt :: Display for ParseError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "failed parsing encoding: {} at byte-index {} in {:?}" , self . kind , self . split_point , self . data ,) } }
};
}
