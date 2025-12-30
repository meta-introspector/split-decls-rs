// Generated macro for session_auth (function)
macro_rules! Depcrate_authenticationsession_auth {
() => {
// Module: crate::authentication
// Provides: {"session_auth"}
// Dependencies: {}
# [test] fn session_auth () { let addr = crate :: address :: read_session_address () . unwrap () ; if ! addr . starts_with ("unix:path=") { return ; } let path = std :: path :: Path :: new (& addr ["unix:path=" . len () ..]) ; let stream = std :: os :: unix :: net :: UnixStream :: connect (& path) . unwrap () ; let mut reader = std :: io :: BufReader :: new (& stream) ; assert ! (Authentication :: blocking (& mut reader , & mut & stream , true) . unwrap ()) ; }
};
}
