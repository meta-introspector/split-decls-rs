// Generated macro for impl_276 (impl)
macro_rules! Depcrate_symbol_indeximpl_276 {
() => {
// Module: crate::symbol_index
// Provides: {"impl_276"}
// Dependencies: {}
impl Query { pub fn new (query : String) -> Query { let lowercased = query . to_lowercase () ; Query { query , lowercased , only_types : false , libs : false , mode : SearchMode :: Fuzzy , assoc_mode : AssocSearchMode :: Include , case_sensitive : false , exclude_imports : false , } } pub fn only_types (& mut self) { self . only_types = true ; } pub fn libs (& mut self) { self . libs = true ; } pub fn fuzzy (& mut self) { self . mode = SearchMode :: Fuzzy ; } pub fn exact (& mut self) { self . mode = SearchMode :: Exact ; } pub fn prefix (& mut self) { self . mode = SearchMode :: Prefix ; } # [doc = " Specifies whether we want to include associated items in the result."] pub fn assoc_search_mode (& mut self , assoc_mode : AssocSearchMode) { self . assoc_mode = assoc_mode ; } pub fn case_sensitive (& mut self) { self . case_sensitive = true ; } pub fn exclude_imports (& mut self) { self . exclude_imports = true ; } }
};
}
