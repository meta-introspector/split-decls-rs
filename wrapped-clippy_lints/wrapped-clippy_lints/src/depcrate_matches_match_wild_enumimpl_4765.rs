// Generated macro for impl_4765 (impl)
macro_rules! Depcrate_matches_match_wild_enumimpl_4765 {
() => {
// Module: crate::matches::match_wild_enum
// Provides: {"impl_4765"}
// Dependencies: {}
impl < 'a > CommonPrefixSearcher < 'a > { fn with_path (& mut self , path : & 'a [PathSegment < 'a >]) { if let [path @ .. , _] = path { self . with_prefix (path) ; } } fn with_prefix (& mut self , path : & 'a [PathSegment < 'a >]) { match self { Self :: None => * self = Self :: Path (path) , Self :: Path (self_path) if path . iter () . map (| p | p . ident . name) . eq (self_path . iter () . map (| p | p . ident . name)) => { } , Self :: Path (_) => * self = Self :: Mixed , Self :: Mixed => () , } } }
};
}
