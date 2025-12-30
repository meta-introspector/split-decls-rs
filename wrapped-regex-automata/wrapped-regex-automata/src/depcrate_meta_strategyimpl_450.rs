// Generated macro for impl_450 (impl)
macro_rules! Depcrate_meta_strategyimpl_450 {
() => {
// Module: crate::meta::strategy
// Provides: {"impl_450"}
// Dependencies: {}
impl ReverseAnchored { fn new (core : Core) -> Result < ReverseAnchored , Core > { if ! core . info . is_always_anchored_end () { debug ! ("skipping reverse anchored optimization because \
				 the regex is not always anchored at the end") ; return Err (core) ; } if core . info . is_always_anchored_start () { debug ! ("skipping reverse anchored optimization because \
				 the regex is also anchored at the start") ; return Err (core) ; } if ! core . hybrid . is_some () && ! core . dfa . is_some () { debug ! ("skipping reverse anchored optimization because \
				 we don't have a lazy DFA or a full DFA") ; return Err (core) ; } Ok (ReverseAnchored { core }) } # [cfg_attr (feature = "perf-inline" , inline (always))] fn try_search_half_anchored_rev (& self , cache : & mut Cache , input : & Input < '_ > ,) -> Result < Option < HalfMatch > , RetryFailError > { let input = input . clone () . anchored (Anchored :: Yes) ; if let Some (e) = self . core . dfa . get (& input) { trace ! ("using full DFA for reverse anchored search at {:?}" , input . get_span ()) ; e . try_search_half_rev (& input) } else if let Some (e) = self . core . hybrid . get (& input) { trace ! ("using lazy DFA for reverse anchored search at {:?}" , input . get_span ()) ; e . try_search_half_rev (& mut cache . hybrid , & input) } else { unreachable ! ("ReverseAnchored always has a DFA") } } }
};
}
