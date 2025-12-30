// Generated macro for impl_357 (impl)
macro_rules! Depcrate_parser_utilsimpl_357 {
() => {
// Module: crate::parser::utils
// Provides: {"impl_357"}
// Dependencies: {}
impl < T : Display > Display for Spanning < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}. At {}" , self . item , self . span . start) } }
};
}
