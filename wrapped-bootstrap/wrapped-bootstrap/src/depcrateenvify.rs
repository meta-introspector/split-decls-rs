// Generated macro for envify (function)
macro_rules! Depcrateenvify {
() => {
// Module: crate
// Provides: {"envify"}
// Dependencies: {}
fn envify (s : & str) -> String { s . chars () . map (| c | match c { '-' => '_' , c => c , }) . flat_map (| c | c . to_uppercase ()) . collect () }
};
}
