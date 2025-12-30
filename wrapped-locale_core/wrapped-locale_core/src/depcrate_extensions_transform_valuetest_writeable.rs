// Generated macro for test_writeable (function)
macro_rules! Depcrate_extensions_transform_valuetest_writeable {
() => {
// Module: crate::extensions::transform::value
// Provides: {"test_writeable"}
// Dependencies: {}
# [test] fn test_writeable () { use writeable :: assert_writeable_eq ; let hybrid = "hybrid" . parse () . unwrap () ; let foobar = "foobar" . parse () . unwrap () ; assert_writeable_eq ! (Value :: default () , "true") ; assert_writeable_eq ! (Value :: from_short_slice_unchecked (vec ! [hybrid] . into ()) , "hybrid") ; assert_writeable_eq ! (Value :: from_short_slice_unchecked (vec ! [hybrid , foobar] . into ()) , "hybrid-foobar") ; }
};
}
