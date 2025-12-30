// Generated macro for impl_385 (impl)
macro_rules! Depcrate_memmem_searcherimpl_385 {
() => {
// Module: crate::memmem::searcher
// Provides: {"impl_385"}
// Dependencies: {}
impl SearcherRev { # [doc = " Creates a new searcher for finding occurrences of the given needle in"] # [doc = " reverse. That is, it reports the last (instead of the first) occurrence"] # [doc = " of a needle in a haystack."] # [inline] pub (crate) fn new (needle : & [u8]) -> SearcherRev { let kind = if needle . len () <= 1 { if needle . is_empty () { trace ! ("building empty reverse substring searcher") ; SearcherRevKind :: Empty } else { trace ! ("building one-byte reverse substring searcher") ; debug_assert_eq ! (1 , needle . len ()) ; SearcherRevKind :: OneByte { needle : needle [0] } } } else { trace ! ("building scalar two-way reverse substring searcher") ; let finder = twoway :: FinderRev :: new (needle) ; SearcherRevKind :: TwoWay { finder } } ; let rabinkarp = rabinkarp :: FinderRev :: new (needle) ; SearcherRev { kind , rabinkarp } } # [doc = " Searches the given haystack for the last occurrence of the given"] # [doc = " needle. The needle given should be the same as the needle that this"] # [doc = " finder was initialized with."] # [inline] pub (crate) fn rfind (& self , haystack : & [u8] , needle : & [u8] ,) -> Option < usize > { if haystack . len () < needle . len () { return None ; } match self . kind { SearcherRevKind :: Empty => Some (haystack . len ()) , SearcherRevKind :: OneByte { needle } => { crate :: memrchr (needle , haystack) } SearcherRevKind :: TwoWay { ref finder } => { if rabinkarp :: is_fast (haystack , needle) { self . rabinkarp . rfind (haystack , needle) } else { finder . rfind (haystack , needle) } } } } }
};
}
