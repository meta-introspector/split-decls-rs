// Generated macro for newtype_struct (function)
macro_rules! Depcrate_serde_testsnewtype_struct {
() => {
// Module: crate::serde_tests
// Provides: {"newtype_struct"}
// Dependencies: {}
# [test] fn newtype_struct () { let newtype = NewtypeStruct (NewtypeInner (34 , 32 , 13)) ; let comparison = & [Event :: StartArray (Some (3)) , Event :: Integer (34 . into ()) , Event :: Integer (32 . into ()) , Event :: Integer (13 . into ()) , Event :: EndCollection ,] ; assert_roundtrip (newtype , comparison , true) ; }
};
}
