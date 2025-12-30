// Generated macro for extract_lit (function)
macro_rules! Depcrate_attrextract_lit {
() => {
// Module: crate::attr
// Provides: {"extract_lit"}
// Dependencies: {}
# [doc = " Extract a `lit` in `NormMeta::Lit(<lit>)`."] fn extract_lit (meta : NormMeta) -> Option < Lit > { if let NormMeta :: Lit (lit) = meta { Some (lit) } else { None } }
};
}
