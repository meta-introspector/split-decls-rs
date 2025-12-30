// Generated macro for is_non_exhaustive (function)
macro_rules! Depcrate_parseis_non_exhaustive {
() => {
// Module: crate::parse
// Provides: {"is_non_exhaustive"}
// Dependencies: {}
fn is_non_exhaustive (attrs : & [Attribute]) -> bool { for attr in attrs { if attr . path () . is_ident ("non_exhaustive") { return true ; } } false }
};
}
