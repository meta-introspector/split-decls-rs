// Generated macro for impl_266 (impl)
macro_rules! Depcrate_server_errorimpl_266 {
() => {
// Module: crate::server::error
// Provides: {"impl_266"}
// Dependencies: {}
impl Display for AppError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { AppError :: RoutingError (msg) => write ! (f , "Routing error: {}" , msg) , AppError :: ParseError (msg) => write ! (f , "Parse error: {}" , msg) , AppError :: ProviderError (msg) => write ! (f , "Provider error: {}" , msg) , } } }
};
}
