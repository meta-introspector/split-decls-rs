// Generated macro for impl_is_zero_tuples (macro)
macro_rules! Depcrate_vec_is_zeroimpl_is_zero_tuples {
() => {
// Module: crate::vec::is_zero
// Provides: {"impl_is_zero_tuples"}
// Dependencies: {}
macro_rules ! impl_is_zero_tuples { () => { } ; ($ first_arg : ident $ (,$ rest : ident) *) => { unsafe impl <$ first_arg : IsZero , $ ($ rest : IsZero ,) *> IsZero for ($ first_arg , $ ($ rest ,) *) { # [inline] fn is_zero (& self) -> bool { # [allow (non_snake_case)] let ($ first_arg , $ ($ rest ,) *) = self ; $ first_arg . is_zero () $ (&& $ rest . is_zero ()) * } } impl_is_zero_tuples ! ($ ($ rest) ,*) ; } }
};
}
