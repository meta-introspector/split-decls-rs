// Generated macro for lower_ascii_with_params (function)
macro_rules! Depcratelower_ascii_with_params {
() => {
// Module: crate
// Provides: {"lower_ascii_with_params"}
// Dependencies: {}
fn lower_ascii_with_params (s : & str , semi : usize , params : & [IndexedPair]) -> String { let mut owned = s . to_owned () ; owned [.. semi] . make_ascii_lowercase () ; for & (name , value) in params { owned [range (name)] . make_ascii_lowercase () ; if & owned [range (name)] == "charset" { owned [range (value)] . make_ascii_lowercase () ; } } owned }
};
}
