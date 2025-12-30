// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let fb_css_minified = FB_CSS . repeat (10) ; run_benchmark_group (| group | { group . register_benchmark ("css-parse-fb" , | | { | | StyleSheet :: parse (& fb_css_minified , ParserOptions :: default ()) . unwrap () }) ; }) ; }
};
}
