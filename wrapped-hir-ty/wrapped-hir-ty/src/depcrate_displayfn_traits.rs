// Generated macro for fn_traits (function)
macro_rules! Depcrate_displayfn_traits {
() => {
// Module: crate::display
// Provides: {"fn_traits"}
// Dependencies: {}
fn fn_traits (db : & dyn DefDatabase , trait_ : TraitId) -> impl Iterator < Item = TraitId > + '_ { let krate = trait_ . lookup (db) . container . krate () ; utils :: fn_traits (db , krate) }
};
}
