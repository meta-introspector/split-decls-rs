// Generated macro for merge_arg_attributes (function)
macro_rules! Depcratemerge_arg_attributes {
() => {
// Module: crate
// Provides: {"merge_arg_attributes"}
// Dependencies: {}
fn merge_arg_attributes (dest : & mut Vec < Attribute > , source : & [Attribute]) { for s in source . iter () { if ! dest . contains (s) { dest . push (s . clone ()) } } }
};
}
