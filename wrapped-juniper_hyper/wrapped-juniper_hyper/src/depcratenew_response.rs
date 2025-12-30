// Generated macro for new_response (function)
macro_rules! Depcratenew_response {
() => {
// Module: crate
// Provides: {"new_response"}
// Dependencies: {}
fn new_response (code : StatusCode) -> Response < String > { let mut r = Response :: new (String :: new ()) ; * r . status_mut () = code ; r }
};
}
