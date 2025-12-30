// Generated macro for handle_request (function)
macro_rules! Depcratehandle_request {
() => {
// Module: crate
// Provides: {"handle_request"}
// Dependencies: {}
fn handle_request (request : tiny_http :: Request) { eprintln ! ("{request:?}") ; let now_utc = time :: OffsetDateTime :: now_utc () ; let text = format ! ("Hello from Hermit! 🦀\nThe current date and time in UTC is {now_utc}.") ; let response = tiny_http :: Response :: from_string (text) ; request . respond (response) . unwrap () ; }
};
}
