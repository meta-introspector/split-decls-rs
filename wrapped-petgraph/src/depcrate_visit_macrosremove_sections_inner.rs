// Generated macro for remove_sections_inner (macro)
macro_rules! Depcrate_visit_macrosremove_sections_inner {
() => {
// Module: crate::visit::macros
// Provides: {"remove_sections_inner"}
// Dependencies: {}
macro_rules ! remove_sections_inner { ([$ ($ stack : tt) *]) => { $ ($ stack) * } ; ([$ ($ stack : tt) *] @ escape $ _x : tt $ ($ t : tt) *) => { remove_sections_inner ! ([$ ($ stack) *] $ ($ t) *) ; } ; ([$ ($ stack : tt) *] @ section $ x : ident $ ($ t : tt) *) => { remove_sections_inner ! ([$ ($ stack) *] $ ($ t) *) ; } ; ([$ ($ stack : tt) *] $ t : tt $ ($ tail : tt) *) => { remove_sections_inner ! ([$ ($ stack) * $ t] $ ($ tail) *) ; } ; }
};
}
