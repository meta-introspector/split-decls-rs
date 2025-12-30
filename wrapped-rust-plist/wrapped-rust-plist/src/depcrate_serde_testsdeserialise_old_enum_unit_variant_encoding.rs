// Generated macro for deserialise_old_enum_unit_variant_encoding (function)
macro_rules! Depcrate_serde_testsdeserialise_old_enum_unit_variant_encoding {
() => {
// Module: crate::serde_tests
// Provides: {"deserialise_old_enum_unit_variant_encoding"}
// Dependencies: {}
# [test] fn deserialise_old_enum_unit_variant_encoding () { # [derive (Debug , Deserialize , Eq , PartialEq , Serialize)] enum Foo { Bar , Baz , } let events = & [Event :: StartDictionary (Some (1)) , Event :: String ("Baz" . into ()) , Event :: String ("" . into ()) , Event :: EndCollection ,] ; let mut de = new_deserializer (events . to_vec ()) ; let obj = Foo :: deserialize (& mut de) . unwrap () ; assert_eq ! (obj , Foo :: Baz) ; }
};
}
