// Generated macro for impl_307 (impl)
macro_rules! Depcrate_common_cache_controlimpl_307 {
() => {
// Module: crate::common::cache_control
// Provides: {"impl_307"}
// Dependencies: {}
impl fmt :: Display for Fmt < '_ > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let if_flag = | f : Flags , dir : Directive | { if self . 0 . flags . contains (f) { Some (dir) } else { None } } ; let slice = & [if_flag (Flags :: NO_CACHE , Directive :: NoCache) , if_flag (Flags :: NO_STORE , Directive :: NoStore) , if_flag (Flags :: NO_TRANSFORM , Directive :: NoTransform) , if_flag (Flags :: ONLY_IF_CACHED , Directive :: OnlyIfCached) , if_flag (Flags :: MUST_REVALIDATE , Directive :: MustRevalidate) , if_flag (Flags :: PUBLIC , Directive :: Public) , if_flag (Flags :: PRIVATE , Directive :: Private) , if_flag (Flags :: IMMUTABLE , Directive :: Immutable) , if_flag (Flags :: MUST_UNDERSTAND , Directive :: MustUnderstand) , if_flag (Flags :: PROXY_REVALIDATE , Directive :: ProxyRevalidate) , self . 0 . max_age . as_ref () . map (| s | Directive :: MaxAge (s . as_u64 ())) , self . 0 . max_stale . as_ref () . map (| s | Directive :: MaxStale (s . as_u64 ())) , self . 0 . min_fresh . as_ref () . map (| s | Directive :: MinFresh (s . as_u64 ())) , self . 0 . s_max_age . as_ref () . map (| s | Directive :: SMaxAge (s . as_u64 ())) ,] ; let iter = slice . iter () . filter_map (| o | * o) ; csv :: fmt_comma_delimited (f , iter) } }
};
}
