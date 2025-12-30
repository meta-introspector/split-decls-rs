// Generated macro for rev_for_each_ident (macro)
macro_rules! Depcrate_tuple_implrev_for_each_ident {
() => {
// Module: crate::tuple_impl
// Provides: {"rev_for_each_ident"}
// Dependencies: {}
macro_rules ! rev_for_each_ident { ($ m : ident ,) => { } ; ($ m : ident , $ i0 : ident , $ ($ i : ident ,) *) => { rev_for_each_ident ! ($ m , $ ($ i ,) *) ; $ m ! ($ i0) ; } ; }
};
}
