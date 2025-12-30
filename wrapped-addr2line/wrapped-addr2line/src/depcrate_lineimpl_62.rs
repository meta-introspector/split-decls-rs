// Generated macro for impl_62 (impl)
macro_rules! Depcrate_lineimpl_62 {
() => {
// Module: crate::line
// Provides: {"impl_62"}
// Dependencies: {}
impl LazyLines { pub (crate) fn new () -> Self { LazyLines (LazyResult :: new ()) } pub (crate) fn borrow < R : gimli :: Reader > (& self , dw_unit : gimli :: UnitRef < R > , ilnp : & gimli :: IncompleteLineProgram < R , R :: Offset > ,) -> Result < & Lines , Error > { self . 0 . get_or_init (| | Lines :: parse (dw_unit , ilnp . clone ())) . as_ref () . map_err (Error :: clone) } }
};
}
