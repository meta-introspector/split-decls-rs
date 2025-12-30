// Generated macro for impl_570 (impl)
macro_rules! Depcrate_output_detailsimpl_570 {
() => {
// Module: crate::output::details
// Provides: {"impl_570"}
// Dependencies: {}
impl < 'a > Iterator for TableIter < 'a > { type Item = TextCell ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| row | { let mut cell = if let Some (cells) = row . cells { self . table . render (cells) } else { let mut cell = TextCell :: default () ; cell . add_spaces (self . total_width) ; cell } ; for tree_part in self . tree_trunk . new_row (row . tree) { cell . push (self . tree_style . paint (tree_part . ascii_art ()) , 4) ; } cell . append (row . name) ; cell }) } }
};
}
