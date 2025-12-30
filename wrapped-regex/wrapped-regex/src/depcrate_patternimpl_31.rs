// Generated macro for impl_31 (impl)
macro_rules! Depcrate_patternimpl_31 {
() => {
// Module: crate::pattern
// Provides: {"impl_31"}
// Dependencies: {}
impl < 'r > Pattern for & 'r Regex { type Searcher < 't > = RegexSearcher < 'r , 't > ; fn into_searcher < 't > (self , haystack : & 't str) -> RegexSearcher < 'r , 't > { RegexSearcher { haystack , it : self . find_iter (haystack) , last_step_end : 0 , next_match : None , } } fn as_utf8_pattern < 'p > (& 'p self) -> Option < Utf8Pattern < 'p > > { None } }
};
}
