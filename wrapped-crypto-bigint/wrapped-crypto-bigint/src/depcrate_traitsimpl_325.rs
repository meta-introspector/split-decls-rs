// Generated macro for impl_325 (impl)
macro_rules! Depcrate_traitsimpl_325 {
() => {
// Module: crate::traits
// Provides: {"impl_325"}
// Dependencies: {}
impl < T : ConditionallySelectable > ConstantTimeSelect for T { # [inline (always)] fn ct_select (a : & Self , b : & Self , choice : Choice) -> Self { T :: conditional_select (a , b , choice) } # [inline (always)] fn ct_assign (& mut self , other : & Self , choice : Choice) { self . conditional_assign (other , choice) } # [inline (always)] fn ct_swap (a : & mut Self , b : & mut Self , choice : Choice) { T :: conditional_swap (a , b , choice) } }
};
}
