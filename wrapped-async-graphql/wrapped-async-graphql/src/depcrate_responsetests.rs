// Generated macro for tests (module)
macro_rules! Depcrate_responsetests {
() => {
// Module: crate::response
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_batch_response_single () { let resp = BatchResponse :: Single (Response :: new (Value :: Boolean (true))) ; assert_eq ! (serde_json :: to_string (& resp) . unwrap () , r#"{"data":true}"#) ; } # [test] fn test_batch_response_batch () { let resp = BatchResponse :: Batch (vec ! [Response :: new (Value :: Boolean (true)) , Response :: new (Value :: String ("1" . to_string ())) ,]) ; assert_eq ! (serde_json :: to_string (& resp) . unwrap () , r#"[{"data":true},{"data":"1"}]"#) ; } }
};
}
