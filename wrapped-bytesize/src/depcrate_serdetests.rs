// Generated macro for tests (module)
macro_rules! Depcrate_serdetests {
() => {
// Module: crate::serde
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use serde :: { Deserialize , Serialize } ; # [test] fn test_serde () { # [derive (Serialize , Deserialize)] struct S { x : ByteSize , } let s = serde_json :: from_str :: < S > (r#"{ "x": "5 B" }"#) . unwrap () ; assert_eq ! (s . x , ByteSize (5)) ; let s = serde_json :: from_str :: < S > (r#"{ "x": 1048576 }"#) . unwrap () ; assert_eq ! (s . x , "1 MiB" . parse ::< ByteSize > () . unwrap ()) ; let s = toml :: from_str :: < S > (r#"x = "2.5 MiB""#) . unwrap () ; assert_eq ! (s . x , "2.5 MiB" . parse ::< ByteSize > () . unwrap ()) ; let s = toml :: from_str :: < S > (r#"x = "9223372036854775807""#) . unwrap () ; assert_eq ! (s . x , "9223372036854775807" . parse ::< ByteSize > () . unwrap ()) ; } # [test] fn test_serde_json () { let json = serde_json :: to_string (& ByteSize :: mib (1)) . unwrap () ; assert_eq ! (json , "\"1.0 MiB\"") ; let deserialized = serde_json :: from_str :: < ByteSize > (& json) . unwrap () ; assert_eq ! (deserialized . 0 , 1048576) ; } }
};
}
