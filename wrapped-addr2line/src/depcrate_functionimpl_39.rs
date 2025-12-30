// Generated macro for impl_39 (impl)
macro_rules! Depcrate_functionimpl_39 {
() => {
// Module: crate::function
// Provides: {"impl_39"}
// Dependencies: {}
impl < R : gimli :: Reader > LazyFunctions < R > { pub (crate) fn new () -> Self { LazyFunctions (LazyResult :: new ()) } pub (crate) fn borrow (& self , unit : gimli :: UnitRef < R >) -> Result < & Functions < R > , Error > { self . 0 . get_or_init (| | Functions :: parse (unit)) . as_ref () . map_err (Error :: clone) } }
};
}
