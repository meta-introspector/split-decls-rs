// Generated macro for test_dedup (function)
macro_rules! Depcrate_teststest_dedup {
() => {
// Module: crate::tests
// Provides: {"test_dedup"}
// Dependencies: {}
# [test] fn test_dedup () { let mut dupes : SmallVec < i32 , 5 > = SmallVec :: from (& [1 , 1 , 2 , 3 , 3]) ; dupes . dedup () ; assert_eq ! (&* dupes , & [1 , 2 , 3]) ; let mut empty : SmallVec < i32 , 5 > = SmallVec :: new () ; empty . dedup () ; assert ! (empty . is_empty ()) ; let mut all_ones : SmallVec < i32 , 5 > = SmallVec :: from (& [1 , 1 , 1 , 1 , 1]) ; all_ones . dedup () ; assert_eq ! (all_ones . len () , 1) ; let mut no_dupes : SmallVec < i32 , 5 > = SmallVec :: from (& [1 , 2 , 3 , 4 , 5]) ; no_dupes . dedup () ; assert_eq ! (no_dupes . len () , 5) ; }
};
}
