// Generated macro for impl_541 (impl)
macro_rules! Depcrate_expression_select_byimpl_541 {
() => {
// Module: crate::expression::select_by
// Provides: {"impl_541"}
// Dependencies: {}
impl < T , DB > Clone for SelectBy < T , DB > where DB : Backend , T : Selectable < DB > , { fn clone (& self) -> Self { Self { selection : T :: construct_selection () , p : std :: marker :: PhantomData , } } }
};
}
