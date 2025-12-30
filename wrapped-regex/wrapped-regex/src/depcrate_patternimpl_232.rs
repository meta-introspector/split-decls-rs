// Generated macro for impl_232 (impl)
macro_rules! Depcrate_patternimpl_232 {
() => {
// Module: crate::pattern
// Provides: {"impl_232"}
// Dependencies: {}
# [cfg (feature = "pattern")] unsafe impl < 'r , 't > Searcher < 't > for RegexSearcher < 'r , 't > { # [inline] fn haystack (& self) -> & 't str { self . haystack } # [inline] fn next (& mut self) -> SearchStep { if let Some ((s , e)) = self . next_match { self . next_match = None ; self . last_step_end = e ; return SearchStep :: Match (s , e) ; } match self . it . next () { None => { if self . last_step_end < self . haystack () . len () { let last = self . last_step_end ; self . last_step_end = self . haystack () . len () ; SearchStep :: Reject (last , self . haystack () . len ()) } else { SearchStep :: Done } } Some ((s , e)) => { if s == self . last_step_end { self . last_step_end = e ; SearchStep :: Match (s , e) } else { self . next_match = Some ((s , e)) ; let last = self . last_step_end ; self . last_step_end = s ; SearchStep :: Reject (last , s) } } } } }
};
}
