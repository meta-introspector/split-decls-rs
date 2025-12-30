// Generated macro for impl_231 (impl)
macro_rules! Depcrate_patternimpl_231 {
() => {
// Module: crate::pattern
// Provides: {"impl_231"}
// Dependencies: {}
# [cfg (feature = "pattern")] impl < 'r , 't > Pattern < 't > for & 'r Regex { type Searcher = RegexSearcher < 'r , 't > ; fn into_searcher (self , haystack : & 't str) -> RegexSearcher < 'r , 't > { RegexSearcher { haystack : haystack , it : self . find_iter (haystack) , last_step_end : 0 , next_match : None , } } }
};
}
