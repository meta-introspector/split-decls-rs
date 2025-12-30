// Generated macro for impl_310 (impl)
macro_rules! Depcrate_common_cache_controlimpl_310 {
() => {
// Module: crate::common::cache_control
// Provides: {"impl_310"}
// Dependencies: {}
impl fmt :: Display for Directive { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (match * self { Directive :: NoCache => "no-cache" , Directive :: NoStore => "no-store" , Directive :: NoTransform => "no-transform" , Directive :: OnlyIfCached => "only-if-cached" , Directive :: MaxAge (secs) => return write ! (f , "max-age={}" , secs) , Directive :: MaxStale (secs) => return write ! (f , "max-stale={}" , secs) , Directive :: MinFresh (secs) => return write ! (f , "min-fresh={}" , secs) , Directive :: MustRevalidate => "must-revalidate" , Directive :: MustUnderstand => "must-understand" , Directive :: Public => "public" , Directive :: Private => "private" , Directive :: Immutable => "immutable" , Directive :: ProxyRevalidate => "proxy-revalidate" , Directive :: SMaxAge (secs) => return write ! (f , "s-maxage={}" , secs) , } , f ,) } }
};
}
