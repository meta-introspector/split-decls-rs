// Generated macro for test (module)
macro_rules! Depcrate_features_arbitrarytest {
() => {
// Module: crate::features::arbitrary
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use arbitrary :: { Arbitrary , Unstructured } ; use crate :: CompactString ; # [test] fn arbitrary_sanity () { let mut data = Unstructured :: new (& [42 ; 50]) ; let compact = CompactString :: arbitrary (& mut data) . expect ("generate a CompactString") ; assert ! (! compact . is_empty ()) ; } # [test] fn arbitrary_inlines_strings () { let mut data = Unstructured :: new (& [42 ; 20]) ; let compact = CompactString :: arbitrary (& mut data) . expect ("generate a CompactString") ; assert ! (! compact . is_heap_allocated ()) ; } }
};
}
