// Generated macro for print_docs (function)
macro_rules! Depcrateprint_docs {
() => {
// Module: crate
// Provides: {"print_docs"}
// Dependencies: {}
fn print_docs (src : & mut String , docs : & Docs) { if let Some (docs) = & docs . contents { let lines = docs . trim () . lines () . map (| line | format ! ("/// {line}")) . collect :: < Vec < _ > > () . join ("\n") ; uwrite ! (src , "{}" , lines) } }
};
}
