// Generated macro for enum_variant_types (function)
macro_rules! Depcrate_serde_testsenum_variant_types {
() => {
// Module: crate::serde_tests
// Provides: {"enum_variant_types"}
// Dependencies: {}
# [test] fn enum_variant_types () { # [derive (Debug , Deserialize , Eq , PartialEq , Serialize)] enum Foo { Unit , Newtype (u32) , Tuple (u32 , String) , Struct { v : u32 , s : String } , } let expected = & [Event :: String ("Unit" . into ())] ; assert_roundtrip (Foo :: Unit , expected , true) ; let expected = & [Event :: StartDictionary (Some (1)) , Event :: String ("Newtype" . into ()) , Event :: Integer (42 . into ()) , Event :: EndCollection ,] ; assert_roundtrip (Foo :: Newtype (42) , expected , true) ; let expected = & [Event :: StartDictionary (Some (1)) , Event :: String ("Tuple" . into ()) , Event :: StartArray (Some (2)) , Event :: Integer (42 . into ()) , Event :: String ("bar" . into ()) , Event :: EndCollection , Event :: EndCollection ,] ; assert_roundtrip (Foo :: Tuple (42 , "bar" . into ()) , expected , true) ; let expected = & [Event :: StartDictionary (Some (1)) , Event :: String ("Struct" . into ()) , Event :: StartDictionary (None) , Event :: String ("v" . into ()) , Event :: Integer (42 . into ()) , Event :: String ("s" . into ()) , Event :: String ("bar" . into ()) , Event :: EndCollection , Event :: EndCollection ,] ; assert_roundtrip (Foo :: Struct { v : 42 , s : "bar" . into () , } , expected , true ,) ; }
};
}
