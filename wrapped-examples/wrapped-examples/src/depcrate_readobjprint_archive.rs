// Generated macro for print_archive (function)
macro_rules! Depcrate_readobjprint_archive {
() => {
// Module: crate::readobj
// Provides: {"print_archive"}
// Dependencies: {}
fn print_archive (p : & mut Printer < '_ > , data : & [u8]) { if let Some (archive) = ArchiveFile :: parse (data) . print_err (p) { write ! (p . w () , "Format: Archive ({:?})" , archive . kind ()) . unwrap () ; if archive . is_thin () { write ! (p . w () , " (thin)") . unwrap () ; } p . blank () ; for member in archive . members () { if let Some (member) = member . print_err (p) { p . blank () ; p . field_inline_string ("Member" , member . name ()) ; if member . is_thin () { p . field ("Size" , member . size ()) ; } else if let Some (data) = member . data (data) . print_err (p) { print_object (p , data , & []) ; } } } if let Some (symbols) = archive . symbols () . print_err (p) . flatten () { p . blank () ; for symbol in symbols { if let Some (symbol) = symbol . print_err (p) { p . group ("Symbol" , | p | { p . field_inline_string ("Name" , symbol . name ()) ; let offset = symbol . offset () ; if let Some (member) = archive . member (offset) . print_err (p) { p . field_inline_string ("Member" , member . name ()) ; } }) ; } } } } }
};
}
