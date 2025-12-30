// Generated macro for test_scoped_map2 (function)
macro_rules! Depcrate_adt_maptest_scoped_map2 {
() => {
// Module: crate::adt::map
// Provides: {"test_scoped_map2"}
// Dependencies: {}
# [test] fn test_scoped_map2 () { let mut map : ScopedMap < usize , usize > = ScopedMap :: new () ; map . push () ; map . insert (& 1 , & 1) ; map . push () ; map . insert (& 1 , & 2) ; map . insert (& 2 , & 3) ; map . push () ; let flat = map . flatten () ; assert ! (flat . contains_key (& 1)) ; assert ! (flat . contains_key (& 2)) ; assert ! (! flat . contains_key (& 3)) ; assert_eq ! (* flat . get (& 1) . unwrap () , 2) ; }
};
}
