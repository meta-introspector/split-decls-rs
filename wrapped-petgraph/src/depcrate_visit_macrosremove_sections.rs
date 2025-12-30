// Generated macro for remove_sections (macro)
macro_rules! Depcrate_visit_macrosremove_sections {
() => {
// Module: crate::visit::macros
// Provides: {"remove_sections"}
// Dependencies: {}
macro_rules ! remove_sections { ([$ ($ stack : tt) *]) => { $ ($ stack) * } ; ([$ ($ stack : tt) *] { $ ($ tail : tt) * }) => { $ ($ stack) * { remove_sections_inner ! ([] $ ($ tail) *) ; } } ; ([$ ($ stack : tt) *] $ t : tt $ ($ tail : tt) *) => { remove_sections ! ([$ ($ stack) * $ t] $ ($ tail) *) ; } ; }
};
}
