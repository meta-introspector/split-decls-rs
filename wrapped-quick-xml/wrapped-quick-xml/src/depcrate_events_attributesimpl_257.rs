// Generated macro for impl_257 (impl)
macro_rules! Depcrate_events_attributesimpl_257 {
() => {
// Module: crate::events::attributes
// Provides: {"impl_257"}
// Dependencies: {}
impl < 'a > Iterator for Attributes < 'a > { type Item = Result < Attribute < 'a > , AttrError > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { match self . state . next (self . bytes) { None => None , Some (Ok (a)) => Some (Ok (a . map (| range | & self . bytes [range]) . into ())) , Some (Err (e)) => Some (Err (e)) , } } }
};
}
