// Generated macro for impl_570 (impl)
macro_rules! Depcrate_uriimpl_570 {
() => {
// Module: crate::uri
// Provides: {"impl_570"}
// Dependencies: {}
impl fmt :: Display for Uri { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (scheme) = self . scheme () { write ! (f , "{}://" , scheme) ? ; } if let Some (authority) = self . authority () { write ! (f , "{}" , authority) ? ; } write ! (f , "{}" , self . path ()) ? ; if let Some (query) = self . query () { write ! (f , "?{}" , query) ? ; } Ok (()) } }
};
}
