// Generated macro for get_key (function)
macro_rules! Depcrateget_key {
() => {
// Module: crate
// Provides: {"get_key"}
// Dependencies: {}
async fn get_key (path : Path < String > , state : State < AppState >) -> impl IntoResponse { let state = state . db . read () . unwrap () ; if let Some (value) = state . get (& * path) . cloned () { Ok (value) } else { Err (StatusCode :: NOT_FOUND) } }
};
}
