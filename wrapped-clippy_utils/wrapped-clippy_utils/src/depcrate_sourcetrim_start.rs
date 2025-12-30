// Generated macro for trim_start (function)
macro_rules! Depcrate_sourcetrim_start {
() => {
// Module: crate::source
// Provides: {"trim_start"}
// Dependencies: {}
fn trim_start (sm : & SourceMap , sp : Range < BytePos >) -> Range < BytePos > { map_range (sm , sp . clone () , | _ , src , range | { let src = src . get (range . clone ()) ? ; Some (range . start + (src . len () - src . trim_start () . len ()) .. range . end) }) . unwrap_or (sp) }
};
}
