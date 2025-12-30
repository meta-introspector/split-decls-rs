// Generated macro for current_node (function)
macro_rules! Depcrate_tree_builder_rulescurrent_node {
() => {
// Module: crate::tree_builder::rules
// Provides: {"current_node"}
// Dependencies: {}
fn current_node < Handle > (open_elems : & [Handle]) -> & Handle { open_elems . last () . expect ("no current element") }
};
}
