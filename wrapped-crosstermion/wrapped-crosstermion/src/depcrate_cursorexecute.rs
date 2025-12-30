// Generated macro for execute (macro)
macro_rules! Depcrate_cursorexecute {
() => {
// Module: crate::cursor
// Provides: {"execute"}
// Dependencies: {}
# [macro_export] macro_rules ! execute { ($ writer : expr $ (, $ command : expr) * $ (,) ?) => { $ crate :: crossterm :: queue ! ($ writer $ (, $ command) *) . and_then (| () | { $ writer . flush () }) } }
};
}
