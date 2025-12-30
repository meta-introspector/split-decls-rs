// Generated macro for check_duplicates (function)
macro_rules! Depcratecheck_duplicates {
() => {
// Module: crate
// Provides: {"check_duplicates"}
// Dependencies: {}
fn check_duplicates (entries : & [Entry]) -> parse :: Result < () > { let mut keys = HashSet :: new () ; for entry in entries { if let Some (first) = entry . key . parsed . first () { if ! keys . insert (first) { return Err (Error :: new_spanned (& entry . key . expr [0] , "duplicate key")) ; } } } Ok (()) }
};
}
