// Generated macro for envify (function)
macro_rules! Depcrateenvify {
() => {
// Module: crate
// Provides: {"envify"}
// Dependencies: {}
fn envify (name : & str) -> String { name . chars () . map (| c | c . to_ascii_uppercase ()) . map (| c | if c == '-' { '_' } else { c }) . collect () }
};
}
