// Generated macro for impl_464 (impl)
macro_rules! Depcrate_consteval_tests_method_resolutionimpl_464 {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"impl_464"}
// Dependencies: {}
impl < F > MethodCandidateCallback for F where F : FnMut (ReceiverAdjustments , AssocItemId , bool) -> ControlFlow < () > , { fn on_inherent_method (& mut self , adjustments : ReceiverAdjustments , item : AssocItemId , is_visible : bool ,) -> ControlFlow < () > { self (adjustments , item , is_visible) } fn on_trait_method (& mut self , adjustments : ReceiverAdjustments , item : AssocItemId , is_visible : bool ,) -> ControlFlow < () > { self (adjustments , item , is_visible) } }
};
}
