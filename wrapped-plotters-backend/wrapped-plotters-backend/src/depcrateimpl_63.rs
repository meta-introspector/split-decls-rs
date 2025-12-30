// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
impl < E : Error + Send + Sync > std :: fmt :: Display for DrawingErrorKind < E > { fn fmt (& self , fmt : & mut std :: fmt :: Formatter) -> Result < () , std :: fmt :: Error > { match self { DrawingErrorKind :: DrawingError (e) => write ! (fmt , "Drawing backend error: {}" , e) , DrawingErrorKind :: FontError (e) => write ! (fmt , "Font loading error: {}" , e) , } } }
};
}
