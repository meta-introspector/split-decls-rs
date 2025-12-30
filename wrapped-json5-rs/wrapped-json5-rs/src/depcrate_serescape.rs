// Generated macro for escape (function)
macro_rules! Depcrate_serescape {
() => {
// Module: crate::ser
// Provides: {"escape"}
// Dependencies: {}
fn escape (v : & str) -> String { v . chars () . flat_map (| c | match c { '"' => vec ! ['\\' , c] , '\n' => vec ! ['\\' , 'n'] , '\r' => vec ! ['\\' , 'r'] , '\t' => vec ! ['\\' , 't'] , '/' => vec ! ['\\' , '/'] , '\\' => vec ! ['\\' , '\\'] , '\u{0008}' => vec ! ['\\' , 'b'] , '\u{000c}' => vec ! ['\\' , 'f'] , c => vec ! [c] , }) . collect () }
};
}
