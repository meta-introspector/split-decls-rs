// Generated macro for impl_474 (impl)
macro_rules! Depcrate_drawing_areaimpl_474 {
() => {
// Module: crate::drawing::area
// Provides: {"impl_474"}
// Dependencies: {}
impl < E : Error + Send + Sync > std :: fmt :: Display for DrawingAreaErrorKind < E > { fn fmt (& self , fmt : & mut std :: fmt :: Formatter) -> Result < () , std :: fmt :: Error > { match self { DrawingAreaErrorKind :: BackendError (e) => write ! (fmt , "backend error: {}" , e) , DrawingAreaErrorKind :: SharingError => { write ! (fmt , "Multiple backend operation in progress") } DrawingAreaErrorKind :: LayoutError => write ! (fmt , "Bad layout") , } } }
};
}
