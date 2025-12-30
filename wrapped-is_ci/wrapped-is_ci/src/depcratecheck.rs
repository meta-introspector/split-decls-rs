// Generated macro for check (function)
macro_rules! Depcratecheck {
() => {
// Module: crate
// Provides: {"check"}
// Dependencies: {}
fn check (name : & str) -> bool { std :: env :: var (name) . is_ok () }
};
}
