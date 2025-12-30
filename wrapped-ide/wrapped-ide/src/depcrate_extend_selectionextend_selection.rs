// Generated macro for extend_selection (function)
macro_rules! Depcrate_extend_selectionextend_selection {
() => {
// Module: crate::extend_selection
// Provides: {"extend_selection"}
// Dependencies: {}
pub (crate) fn extend_selection (db : & RootDatabase , frange : FileRange) -> TextRange { let sema = Semantics :: new (db) ; let src = sema . parse_guess_edition (frange . file_id) ; try_extend_selection (& sema , src . syntax () , frange) . unwrap_or (frange . range) }
};
}
