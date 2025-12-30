// Generated macro for impl_491 (impl)
macro_rules! Depcrate_meta_wrappersimpl_491 {
() => {
// Module: crate::meta::wrappers
// Provides: {"impl_491"}
// Dependencies: {}
impl DFA { pub (crate) fn none () -> DFA { DFA (None) } pub (crate) fn new (info : & RegexInfo , pre : Option < Prefilter > , nfa : & NFA , nfarev : & NFA ,) -> DFA { DFA (DFAEngine :: new (info , pre , nfa , nfarev)) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn get (& self , _input : & Input < '_ >) -> Option < & DFAEngine > { let engine = self . 0 . as_ref () ? ; Some (engine) } pub (crate) fn is_some (& self) -> bool { self . 0 . is_some () } pub (crate) fn memory_usage (& self) -> usize { self . 0 . as_ref () . map_or (0 , | e | e . memory_usage ()) } }
};
}
