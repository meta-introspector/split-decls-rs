// Generated macro for impl_544 (impl)
macro_rules! Depcrate_expression_select_byimpl_544 {
() => {
// Module: crate::expression::select_by
// Provides: {"impl_544"}
// Dependencies: {}
impl < T , DB > SelectBy < T , DB > where T : Selectable < DB > , DB : Backend , { pub (crate) fn new () -> Self { Self { selection : T :: construct_selection () , p : std :: marker :: PhantomData , } } }
};
}
