// Generated macro for cover_range (function)
macro_rules! Depcrate_highlight_relatedcover_range {
() => {
// Module: crate::highlight_related
// Provides: {"cover_range"}
// Dependencies: {}
fn cover_range (r0 : Option < TextRange > , r1 : Option < TextRange >) -> Option < TextRange > { match (r0 , r1) { (Some (r0) , Some (r1)) => Some (r0 . cover (r1)) , (Some (range) , None) => Some (range) , (None , Some (range)) => Some (range) , (None , None) => None , } }
};
}
