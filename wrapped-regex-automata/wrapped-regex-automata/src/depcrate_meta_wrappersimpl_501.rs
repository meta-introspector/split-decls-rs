// Generated macro for impl_501 (impl)
macro_rules! Depcrate_meta_wrappersimpl_501 {
() => {
// Module: crate::meta::wrappers
// Provides: {"impl_501"}
// Dependencies: {}
impl ReverseDFA { pub (crate) fn none () -> ReverseDFA { ReverseDFA (None) } pub (crate) fn new (info : & RegexInfo , nfarev : & NFA) -> ReverseDFA { ReverseDFA (ReverseDFAEngine :: new (info , nfarev)) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn get (& self , _input : & Input < '_ >) -> Option < & ReverseDFAEngine > { let engine = self . 0 . as_ref () ? ; Some (engine) } pub (crate) fn is_some (& self) -> bool { self . 0 . is_some () } pub (crate) fn memory_usage (& self) -> usize { self . 0 . as_ref () . map_or (0 , | e | e . memory_usage ()) } }
};
}
