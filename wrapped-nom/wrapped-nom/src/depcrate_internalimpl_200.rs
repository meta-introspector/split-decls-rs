// Generated macro for impl_200 (impl)
macro_rules! Depcrate_internalimpl_200 {
() => {
// Module: crate::internal
// Provides: {"impl_200"}
// Dependencies: {}
impl < E > fmt :: Display for Err < E > where E : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Err :: Incomplete (Needed :: Size (u)) => write ! (f , "Parsing requires {} bytes/chars" , u) , Err :: Incomplete (Needed :: Unknown) => write ! (f , "Parsing requires more data") , Err :: Failure (c) => write ! (f , "Parsing Failure: {:?}" , c) , Err :: Error (c) => write ! (f , "Parsing Error: {:?}" , c) , } } }
};
}
