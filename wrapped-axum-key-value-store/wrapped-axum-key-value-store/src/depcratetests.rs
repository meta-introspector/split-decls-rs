// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use axum :: { body :: Body , http :: Request } ; use http_body_util :: BodyExt ; use tower :: ServiceExt ; use super :: * ; # [tokio :: test] async fn get_and_set_value () -> Result < () , Box < dyn std :: error :: Error > > { let app = app () ; let response = app . clone () . oneshot (Request :: get ("/foo") . body (Body :: empty ()) ?) . await ? ; assert_eq ! (response . status () , StatusCode :: NOT_FOUND) ; let response = app . clone () . oneshot (Request :: post ("/foo") . body (Body :: from ("Hello, World!")) ?) . await ? ; assert_eq ! (response . status () , StatusCode :: OK) ; let response = app . oneshot (Request :: get ("/foo") . body (Body :: empty ()) ?) . await ? ; assert_eq ! (response . status () , StatusCode :: OK) ; let body = response . into_body () . collect () . await ? . to_bytes () ; assert_eq ! (body . as_ref () , b"Hello, World!") ; Ok (()) } }
};
}
