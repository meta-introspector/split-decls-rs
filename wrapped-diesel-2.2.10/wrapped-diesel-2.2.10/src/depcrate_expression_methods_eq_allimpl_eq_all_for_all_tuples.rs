// Generated macro for impl_eq_all_for_all_tuples (macro)
macro_rules! Depcrate_expression_methods_eq_allimpl_eq_all_for_all_tuples {
() => {
// Module: crate::expression_methods::eq_all
// Provides: {"impl_eq_all_for_all_tuples"}
// Dependencies: {}
macro_rules ! impl_eq_all_for_all_tuples { ($ ($ unused1 : tt { $ ($ unused2 : tt -> $ Left : ident , $ Right : ident , $ unused3 : tt ,) + }) +) => { $ (impl_eq_all ! (($ ($ Left ,) +) ($ ($ Right ,) +)) ;) + } ; }
};
}
