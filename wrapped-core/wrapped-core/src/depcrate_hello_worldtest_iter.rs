// Generated macro for test_iter (function)
macro_rules! Depcrate_hello_worldtest_iter {
() => {
// Module: crate::hello_world
// Provides: {"test_iter"}
// Dependencies: {}
# [cfg (feature = "export")] # [test] fn test_iter () { use crate :: IterableDataProvider ; use icu_locale_core :: locale ; let ids = HelloWorldProvider . iter_ids () . unwrap () ; assert_eq ! (ids . len () , HelloWorldProvider :: DATA . len ()) ; assert ! (ids . contains (& DataIdentifierCow :: from_borrowed_and_owned (DataMarkerAttributes :: from_str_or_panic ("reverse") , locale ! ("en") . into ()))) ; }
};
}
