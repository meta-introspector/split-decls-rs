// Generated macro for move_if_necessary (function)
macro_rules! Depcratemove_if_necessary {
() => {
// Module: crate
// Provides: {"move_if_necessary"}
// Dependencies: {}
fn move_if_necessary (arg : & str) -> String { if ! arg . is_empty () && arg . chars () . all (char :: is_alphanumeric) { format ! ("std::move({arg})") } else { arg . into () } }
};
}
