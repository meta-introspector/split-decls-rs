// Generated macro for interrupted (function)
macro_rules! Depcrateinterrupted {
() => {
// Module: crate
// Provides: {"interrupted"}
// Dependencies: {}
fn interrupted (err : & io :: Error) -> bool { err . kind () == io :: ErrorKind :: Interrupted }
};
}
