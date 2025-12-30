// Generated macro for test_scoped_map (function)
macro_rules! Depcrate_adt_maptest_scoped_map {
() => {
// Module: crate::adt::map
// Provides: {"test_scoped_map"}
// Dependencies: {}
# [test] fn test_scoped_map () { let mut map : ScopedMap < usize , usize > = ScopedMap :: new () ; assert ! (map . is_empty ()) ; map . push () ; assert_eq ! (map . len () , 1) ; map . insert (& 1 , & 1) ; map . insert (& 2 , & 2) ; map . insert (& 3 , & 3) ; assert_eq ! (map . get (& 1) . unwrap () , 1) ; assert_eq ! (map . get (& 2) . unwrap () , 2) ; assert_eq ! (map . get (& 3) . unwrap () , 3) ; assert ! (map . has (& 1)) ; assert ! (map . has (& 2)) ; assert ! (map . has (& 3)) ; map . push () ; assert ! (map . has (& 1)) ; assert ! (map . has (& 2)) ; assert ! (map . has (& 3)) ; map . insert (& 1 , & 4) ; map . insert (& 2 , & 5) ; map . insert (& 3 , & 6) ; assert_eq ! (map . get (& 1) . unwrap () , 4) ; assert_eq ! (map . get (& 2) . unwrap () , 5) ; assert_eq ! (map . get (& 3) . unwrap () , 6) ; map . pop () ; assert_eq ! (map . get (& 1) . unwrap () , 1) ; assert_eq ! (map . get (& 2) . unwrap () , 2) ; assert_eq ! (map . get (& 3) . unwrap () , 3) ; map . pop () ; assert ! (! map . has (& 1)) ; assert ! (! map . has (& 2)) ; assert ! (! map . has (& 3)) ; }
};
}
