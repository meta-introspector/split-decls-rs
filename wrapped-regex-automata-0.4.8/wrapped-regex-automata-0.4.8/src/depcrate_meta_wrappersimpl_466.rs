// Generated macro for impl_466 (impl)
macro_rules! Depcrate_meta_wrappersimpl_466 {
() => {
// Module: crate::meta::wrappers
// Provides: {"impl_466"}
// Dependencies: {}
impl PikeVMEngine { pub (crate) fn new (info : & RegexInfo , pre : Option < Prefilter > , nfa : & NFA ,) -> Result < PikeVMEngine , BuildError > { let pikevm_config = pikevm :: Config :: new () . match_kind (info . config () . get_match_kind ()) . prefilter (pre) ; let engine = pikevm :: Builder :: new () . configure (pikevm_config) . build_from_nfa (nfa . clone ()) . map_err (BuildError :: nfa) ? ; debug ! ("PikeVM built") ; Ok (PikeVMEngine (engine)) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn is_match (& self , cache : & mut PikeVMCache , input : & Input < '_ > ,) -> bool { self . 0 . is_match (cache . 0 . as_mut () . unwrap () , input . clone ()) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn search_slots (& self , cache : & mut PikeVMCache , input : & Input < '_ > , slots : & mut [Option < NonMaxUsize >] ,) -> Option < PatternID > { self . 0 . search_slots (cache . 0 . as_mut () . unwrap () , input , slots) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn which_overlapping_matches (& self , cache : & mut PikeVMCache , input : & Input < '_ > , patset : & mut PatternSet ,) { self . 0 . which_overlapping_matches (cache . 0 . as_mut () . unwrap () , input , patset ,) } }
};
}
