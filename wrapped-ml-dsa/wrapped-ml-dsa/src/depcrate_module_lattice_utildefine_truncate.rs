// Generated macro for define_truncate (macro)
macro_rules! Depcrate_module_lattice_utildefine_truncate {
() => {
// Module: crate::module_lattice::util
// Provides: {"define_truncate"}
// Dependencies: {}
macro_rules ! define_truncate { ($ from : ident , $ to : ident) => { impl Truncate <$ from > for $ to { fn truncate (x : $ from) -> $ to { unsafe { (x & $ from :: from ($ to :: MAX)) . try_into () . unwrap_unchecked () } } } } ; }
};
}
