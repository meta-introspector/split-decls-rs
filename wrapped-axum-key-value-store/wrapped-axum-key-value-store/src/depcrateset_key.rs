// Generated macro for set_key (function)
macro_rules! Depcrateset_key {
() => {
// Module: crate
// Provides: {"set_key"}
// Dependencies: {}
async fn set_key (Path (path) : Path < String > , state : State < AppState > , value : Bytes) { let mut state = state . db . write () . unwrap () ; state . insert (path , value) ; }
};
}
