// Generated macro for is_archive_symbol (function)
macro_rules! Depcrate_object_readeris_archive_symbol {
() => {
// Module: crate::object_reader
// Provides: {"is_archive_symbol"}
// Dependencies: {}
fn is_archive_symbol (sym : & object :: read :: Symbol < '_ , '_ >) -> bool { if sym . kind () == object :: SymbolKind :: File || sym . kind () == object :: SymbolKind :: Section { return false ; } if ! sym . is_global () { return false ; } if sym . is_undefined () { return false ; } true }
};
}
