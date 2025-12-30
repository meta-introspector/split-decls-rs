// Generated macro for const_eval_cycle_result (function)
macro_rules! Depcrate_constevalconst_eval_cycle_result {
() => {
// Module: crate::consteval
// Provides: {"const_eval_cycle_result"}
// Dependencies: {}
pub (crate) fn const_eval_cycle_result < 'db > (_db : & 'db dyn HirDatabase , _salsa_id : salsa :: Id , _const_id : ConstId , _subst : GenericArgs < 'db > , _trait_env : Option < Arc < TraitEnvironment < 'db > > > ,) -> Result < Const < 'db > , ConstEvalError < 'db > > { Err (ConstEvalError :: MirLowerError (MirLowerError :: Loop)) }
};
}
