// Generated macro for tests (module)
macro_rules! Depcrate_errortests {
() => {
// Module: crate::error
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Error ; # [test] pub fn unknown_kind () { let json = r#"{
            "kind": "unexpected-kind",
            "unexpected-content": "test"
          }"# ; let e : Error = serde_json :: from_str (& json) . unwrap () ; assert ! (matches ! (e , Error :: Unknown)) ; } # [test] pub fn roundtrip () { let e = anyhow :: anyhow ! ("E1") . context ("E2") . context ("E3") ; let s1 = format ! ("{:?}" , e) ; let e : Error = e . into () ; let json = serde_json :: to_string_pretty (& e) . unwrap () ; let e : anyhow :: Error = e . into () ; let s2 = format ! ("{:?}" , e) ; assert_eq ! (s1 , s2) ; let e : Error = serde_json :: from_str (& json) . unwrap () ; let e : anyhow :: Error = e . into () ; let s3 = format ! ("{:?}" , e) ; assert_eq ! (s2 , s3) ; assert_eq ! (r#"{
  "kind": "other",
  "message": "E3",
  "caused-by": [
    "E2",
    "E1"
  ]
}"# , json) ; } }
};
}
