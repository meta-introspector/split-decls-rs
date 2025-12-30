// Generated macro for make_n_ids (function)
macro_rules! Depcratemake_n_ids {
() => {
// Module: crate
// Provides: {"make_n_ids"}
// Dependencies: {}
# [doc = " Returns identifiers to use as bindings in generated code"] fn make_n_ids (n : usize) -> Vec < Ident > { (0 .. n) . map (| i | Ident :: new (& format ! ("__arg{i}") , Span :: call_site ())) . collect () }
};
}
