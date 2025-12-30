// Generated macro for bail (macro)
macro_rules! Depcratebail {
() => {
// Module: crate
// Provides: {"bail"}
// Dependencies: {}
macro_rules ! bail { ($ span : expr , $ ($ tt : tt) *) => { return Err (syn :: Error :: new_spanned ($ span , format ! ($ ($ tt) *))) } ; }
};
}
