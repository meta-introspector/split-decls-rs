// Generated macro for getenv_unwrap (function)
macro_rules! Depcrategetenv_unwrap {
() => {
// Module: crate
// Provides: {"getenv_unwrap"}
// Dependencies: {}
fn getenv_unwrap (v : & str) -> String { match env :: var (v) { Ok (s) => s , Err (..) => fail (& format ! ("environment variable `{}` not defined" , v)) , } }
};
}
