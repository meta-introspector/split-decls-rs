// Generated macro for impl_305 (impl)
macro_rules! Depcrate_common_cache_controlimpl_305 {
() => {
// Module: crate::common::cache_control
// Provides: {"impl_305"}
// Dependencies: {}
impl FromIterator < KnownDirective > for FromIter { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = KnownDirective > , { let mut cc = CacheControl :: new () ; let iter = iter . into_iter () . filter_map (| dir | match dir { KnownDirective :: Known (dir) => Some (dir) , KnownDirective :: Unknown => None , }) ; for directive in iter { match directive { Directive :: NoCache => { cc . flags . insert (Flags :: NO_CACHE) ; } Directive :: NoStore => { cc . flags . insert (Flags :: NO_STORE) ; } Directive :: NoTransform => { cc . flags . insert (Flags :: NO_TRANSFORM) ; } Directive :: OnlyIfCached => { cc . flags . insert (Flags :: ONLY_IF_CACHED) ; } Directive :: MustRevalidate => { cc . flags . insert (Flags :: MUST_REVALIDATE) ; } Directive :: MustUnderstand => { cc . flags . insert (Flags :: MUST_UNDERSTAND) ; } Directive :: Public => { cc . flags . insert (Flags :: PUBLIC) ; } Directive :: Private => { cc . flags . insert (Flags :: PRIVATE) ; } Directive :: Immutable => { cc . flags . insert (Flags :: IMMUTABLE) ; } Directive :: ProxyRevalidate => { cc . flags . insert (Flags :: PROXY_REVALIDATE) ; } Directive :: MaxAge (secs) => { cc . max_age = Some (Duration :: from_secs (secs) . into ()) ; } Directive :: MaxStale (secs) => { cc . max_stale = Some (Duration :: from_secs (secs) . into ()) ; } Directive :: MinFresh (secs) => { cc . min_fresh = Some (Duration :: from_secs (secs) . into ()) ; } Directive :: SMaxAge (secs) => { cc . s_max_age = Some (Duration :: from_secs (secs) . into ()) ; } } } FromIter (cc) } }
};
}
