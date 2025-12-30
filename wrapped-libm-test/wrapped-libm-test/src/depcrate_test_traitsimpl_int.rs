// Generated macro for impl_int (macro)
macro_rules! Depcrate_test_traitsimpl_int {
() => {
// Module: crate::test_traits
// Provides: {"impl_int"}
// Dependencies: {}
macro_rules ! impl_int { ($ ($ ty : ty) ,*) => { $ (impl Hex for $ ty { fn hex (self) -> String { format ! ("{self:#0width$x}" , width = ((Self :: BITS / 4) + 2) as usize) } fn hexf (self) -> String { String :: new () } } impl < Input > $ crate :: CheckOutput < Input > for $ ty where Input : Hex + fmt :: Debug , SpecialCase : MaybeOverride < Input >, { fn validate <'a > (self , expected : Self , input : Input , ctx : &$ crate :: CheckCtx ,) -> TestResult { validate_int (self , expected , input , ctx) } }) * } ; }
};
}
