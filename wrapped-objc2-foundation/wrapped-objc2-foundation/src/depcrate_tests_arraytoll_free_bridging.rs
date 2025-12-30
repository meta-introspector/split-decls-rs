// Generated macro for toll_free_bridging (function)
macro_rules! Depcrate_tests_arraytoll_free_bridging {
() => {
// Module: crate::tests::array
// Provides: {"toll_free_bridging"}
// Dependencies: {}
# [test] # [cfg (feature = "objc2-core-foundation")] # [cfg (not (feature = "gnustep-1-7"))] fn toll_free_bridging () { use objc2_core_foundation :: { CFArray , CFRetained } ; let array = NSArray :: from_retained_slice (& [NSNumber :: new_bool (true)]) ; let cf_array : & CFArray < NSNumber > = array . as_ref () ; assert_eq ! (cf_array . retain_count () , 1) ; let _ : & NSArray < NSNumber > = cf_array . as_ref () ; let cf_array : Retained < CFArray < NSNumber > > = (& array) . into () ; assert_eq ! (cf_array . retain_count () , 2) ; let _ : Retained < NSArray < NSNumber > > = (& cf_array) . into () ; let _ : CFRetained < CFArray < NSNumber > > = (& array) . into () ; }
};
}
