// Generated macro for new (function)
macro_rules! Depcrate_meta_strategynew {
() => {
// Module: crate::meta::strategy
// Provides: {"new"}
// Dependencies: {}
pub (super) fn new (info : & RegexInfo , hirs : & [& Hir] ,) -> Result < Arc < dyn Strategy > , BuildError > { let pre = if info . is_always_anchored_start () { debug ! ("skipping literal extraction since regex is anchored") ; None } else if let Some (pre) = info . config () . get_prefilter () { debug ! ("skipping literal extraction since the caller provided a prefilter") ; Some (pre . clone ()) } else if info . config () . get_auto_prefilter () { let kind = info . config () . get_match_kind () ; let prefixes = crate :: util :: prefilter :: prefixes (kind , hirs) ; if let Some (pre) = Pre :: from_prefixes (info , & prefixes) { debug ! ("found that the regex can be broken down to a literal \
                 search, avoiding the regex engine entirely" ,) ; return Ok (pre) ; } if let Some (pre) = Pre :: from_alternation_literals (info , hirs) { debug ! ("found plain alternation of literals, \
                 avoiding regex engine entirely and using Aho-Corasick") ; return Ok (pre) ; } prefixes . literals () . and_then (| strings | { debug ! ("creating prefilter from {} literals: {:?}" , strings . len () , strings ,) ; Prefilter :: new (kind , strings) }) } else { debug ! ("skipping literal extraction since prefilters were disabled") ; None } ; let mut core = Core :: new (info . clone () , pre . clone () , hirs) ? ; core = match ReverseAnchored :: new (core) { Err (core) => core , Ok (ra) => { debug ! ("using reverse anchored strategy") ; return Ok (Arc :: new (ra)) ; } } ; core = match ReverseSuffix :: new (core , hirs) { Err (core) => core , Ok (rs) => { debug ! ("using reverse suffix strategy") ; return Ok (Arc :: new (rs)) ; } } ; core = match ReverseInner :: new (core , hirs) { Err (core) => core , Ok (ri) => { debug ! ("using reverse inner strategy") ; return Ok (Arc :: new (ri)) ; } } ; debug ! ("using core strategy") ; Ok (Arc :: new (core)) }
};
}
