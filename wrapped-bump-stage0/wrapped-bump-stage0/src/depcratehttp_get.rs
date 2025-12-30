// Generated macro for http_get (function)
macro_rules! Depcratehttp_get {
() => {
// Module: crate
// Provides: {"http_get"}
// Dependencies: {}
fn http_get (url : & str) -> Result < Vec < u8 > , Error > { let mut data = Vec :: new () ; let mut handle = Easy :: new () ; handle . fail_on_error (true) ? ; handle . url (url) ? ; { let mut transfer = handle . transfer () ; transfer . write_function (| new_data | { data . extend_from_slice (new_data) ; Ok (new_data . len ()) }) ? ; transfer . perform () . context (format ! ("failed to fetch {url}")) ? ; } Ok (data) }
};
}
