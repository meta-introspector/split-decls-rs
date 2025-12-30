// Generated macro for handle_rejects (function)
macro_rules! Depcratehandle_rejects {
() => {
// Module: crate
// Provides: {"handle_rejects"}
// Dependencies: {}
# [doc = " Handles all the [`Rejection`]s happening in [`make_graphql_filter()`] to fail fast, if required."] async fn handle_rejects (rej : Rejection) -> Result < reply :: Response , Rejection > { let (status , msg) = if let Some (e) = rej . find :: < FilterError > () { (StatusCode :: BAD_REQUEST , e . to_string ()) } else if let Some (e) = rej . find :: < warp :: reject :: InvalidQuery > () { (StatusCode :: BAD_REQUEST , e . to_string ()) } else if let Some (e) = rej . find :: < BodyDeserializeError > () { (StatusCode :: BAD_REQUEST , e . to_string ()) } else { return Err (rej) ; } ; Ok (http :: Response :: builder () . status (status) . body (msg . into ()) . unwrap ()) }
};
}
