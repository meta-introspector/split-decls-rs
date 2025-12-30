// Generated macro for impl_102 (impl)
macro_rules! Depcrateimpl_102 {
() => {
// Module: crate
// Provides: {"impl_102"}
// Dependencies: {}
impl MultiStrategyBuilder { fn new () -> MultiStrategyBuilder { MultiStrategyBuilder { literals : vec ! [] , map : vec ! [] , longest : 0 } } fn add (& mut self , global_index : usize , literal : String) { if literal . len () > self . longest { self . longest = literal . len () ; } self . map . push (global_index) ; self . literals . push (literal) ; } fn prefix (self) -> PrefixStrategy { PrefixStrategy { matcher : AhoCorasick :: new (& self . literals) . unwrap () , map : self . map , longest : self . longest , } } fn suffix (self) -> SuffixStrategy { SuffixStrategy { matcher : AhoCorasick :: new (& self . literals) . unwrap () , map : self . map , longest : self . longest , } } fn regex_set (self) -> Result < RegexSetStrategy , Error > { let matcher = new_regex_set (self . literals) ? ; let pattern_len = matcher . pattern_len () ; let create : PatternSetPoolFn = Box :: new (move | | PatternSet :: new (pattern_len)) ; Ok (RegexSetStrategy { matcher , map : self . map , patset : Arc :: new (Pool :: new (create)) , }) } fn is_empty (& self) -> bool { self . literals . is_empty () } }
};
}
