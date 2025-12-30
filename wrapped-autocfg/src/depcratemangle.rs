// Generated macro for mangle (function)
macro_rules! Depcratemangle {
() => {
// Module: crate
// Provides: {"mangle"}
// Dependencies: {}
fn mangle (s : & str) -> String { s . chars () . map (| c | match c { 'A' ..= 'Z' | 'a' ..= 'z' | '0' ..= '9' => c , _ => '_' , }) . collect () }
};
}
