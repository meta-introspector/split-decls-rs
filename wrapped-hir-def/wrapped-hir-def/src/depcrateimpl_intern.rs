// Generated macro for impl_intern (macro)
macro_rules! Depcrateimpl_intern {
() => {
// Module: crate
// Provides: {"impl_intern"}
// Dependencies: {}
macro_rules ! impl_intern { ($ id : ident , $ loc : ident , $ intern : ident , $ lookup : ident) => { impl_intern_key ! ($ id , $ loc) ; impl_intern_lookup ! (DefDatabase , $ id , $ loc , $ intern , $ lookup) ; } ; }
};
}
