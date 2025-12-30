// Generated macro for render_error (function)
macro_rules! Depcraterender_error {
() => {
// Module: crate
// Provides: {"render_error"}
// Dependencies: {}
fn render_error < B > (err : GraphQLRequestError < B >) -> Response < String > where B : Body < Error : Display > , { let mut resp = new_response (StatusCode :: BAD_REQUEST) ; * resp . body_mut () = err . to_string () ; resp }
};
}
