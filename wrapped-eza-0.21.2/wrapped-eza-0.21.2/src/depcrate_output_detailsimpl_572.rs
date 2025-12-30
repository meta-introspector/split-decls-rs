// Generated macro for impl_572 (impl)
macro_rules! Depcrate_output_detailsimpl_572 {
() => {
// Module: crate::output::details
// Provides: {"impl_572"}
// Dependencies: {}
impl Iterator for Iter { type Item = TextCell ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| row | { let mut cell = TextCell :: default () ; for tree_part in self . tree_trunk . new_row (row . tree) { cell . push (self . tree_style . paint (tree_part . ascii_art ()) , 4) ; } cell . append (row . name) ; cell }) } }
};
}
