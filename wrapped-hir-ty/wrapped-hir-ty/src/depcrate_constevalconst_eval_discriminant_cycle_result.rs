// Generated macro for const_eval_discriminant_cycle_result (function)
macro_rules! Depcrate_constevalconst_eval_discriminant_cycle_result {
() => {
// Module: crate::consteval
// Provides: {"const_eval_discriminant_cycle_result"}
// Dependencies: {}
pub (crate) fn const_eval_discriminant_cycle_result < 'db > (_db : & 'db dyn HirDatabase , _salsa_id : salsa :: Id , _enum_variant_id : EnumVariantId ,) -> Result < i128 , ConstEvalError < 'db > > { Err (ConstEvalError :: MirLowerError (MirLowerError :: Loop)) }
};
}
