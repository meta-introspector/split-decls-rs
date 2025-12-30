// Generated macro for scan_containers (function)
macro_rules! Depcrate_parsescan_containers {
() => {
// Module: crate::parse
// Provides: {"scan_containers"}
// Dependencies: {}
# [doc = " Returns number of containers scanned."] pub (crate) fn scan_containers (tree : & Tree < Item > , line_start : & mut LineStart < '_ > , options : Options ,) -> usize { let mut i = 0 ; for & node_ix in tree . walk_spine () { match tree [node_ix] . item . body { ItemBody :: BlockQuote (..) => { let save = line_start . clone () ; let _ = line_start . scan_space (3) ; if ! line_start . scan_blockquote_marker () { * line_start = save ; break ; } } ItemBody :: ListItem (indent) => { let save = line_start . clone () ; if ! line_start . scan_space (indent) && ! line_start . is_at_eol () { * line_start = save ; break ; } } ItemBody :: DefinitionListDefinition (indent) => { let save = line_start . clone () ; if ! line_start . scan_space (indent) && ! line_start . is_at_eol () { * line_start = save ; break ; } } ItemBody :: FootnoteDefinition (..) if options . has_gfm_footnotes () => { let save = line_start . clone () ; if ! line_start . scan_space (4) && ! line_start . is_at_eol () { * line_start = save ; break ; } } _ => () , } i += 1 ; } i }
};
}
