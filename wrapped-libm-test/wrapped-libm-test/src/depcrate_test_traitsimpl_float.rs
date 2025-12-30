// Generated macro for impl_float (macro)
macro_rules! Depcrate_test_traitsimpl_float {
() => {
// Module: crate::test_traits
// Provides: {"impl_float"}
// Dependencies: {}
macro_rules ! impl_float { ($ ($ ty : ty) ,*) => { $ (impl Hex for $ ty { fn hex (self) -> String { format ! ("{:#0width$x}" , self . to_bits () , width = ((Self :: BITS / 4) + 2) as usize) } fn hexf (self) -> String { format ! ("{}" , Hexf (self)) } } impl < Input > $ crate :: CheckOutput < Input > for $ ty where Input : Hex + fmt :: Debug , SpecialCase : MaybeOverride < Input >, { fn validate <'a > (self , expected : Self , input : Input , ctx : &$ crate :: CheckCtx ,) -> TestResult { validate_float (self , expected , input , ctx) } }) * } ; }
};
}
