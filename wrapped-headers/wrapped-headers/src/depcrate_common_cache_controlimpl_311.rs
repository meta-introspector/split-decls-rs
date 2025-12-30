// Generated macro for impl_311 (impl)
macro_rules! Depcrate_common_cache_controlimpl_311 {
() => {
// Module: crate::common::cache_control
// Provides: {"impl_311"}
// Dependencies: {}
impl FromStr for KnownDirective { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (KnownDirective :: Known (match s { "no-cache" => Directive :: NoCache , "no-store" => Directive :: NoStore , "no-transform" => Directive :: NoTransform , "only-if-cached" => Directive :: OnlyIfCached , "must-revalidate" => Directive :: MustRevalidate , "public" => Directive :: Public , "private" => Directive :: Private , "immutable" => Directive :: Immutable , "must-understand" => Directive :: MustUnderstand , "proxy-revalidate" => Directive :: ProxyRevalidate , "" => return Err (()) , _ => match s . find ('=') { Some (idx) if idx + 1 < s . len () => { match (& s [.. idx] , (s [idx + 1 ..]) . trim_matches ('"')) { ("max-age" , secs) => secs . parse () . map (Directive :: MaxAge) . map_err (| _ | ()) ? , ("max-stale" , secs) => { secs . parse () . map (Directive :: MaxStale) . map_err (| _ | ()) ? } ("min-fresh" , secs) => { secs . parse () . map (Directive :: MinFresh) . map_err (| _ | ()) ? } ("s-maxage" , secs) => { secs . parse () . map (Directive :: SMaxAge) . map_err (| _ | ()) ? } _unknown => return Ok (KnownDirective :: Unknown) , } } Some (_) | None => return Ok (KnownDirective :: Unknown) , } , })) } }
};
}
