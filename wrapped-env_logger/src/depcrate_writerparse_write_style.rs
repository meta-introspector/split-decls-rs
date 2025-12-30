// Generated macro for parse_write_style (function)
macro_rules! Depcrate_writerparse_write_style {
() => {
// Module: crate::writer
// Provides: {"parse_write_style"}
// Dependencies: {}
fn parse_write_style (spec : & str) -> WriteStyle { match spec { "auto" => WriteStyle :: Auto , "always" => WriteStyle :: Always , "never" => WriteStyle :: Never , _ => Default :: default () , } }
};
}
