// Generated macro for const_eval_query (function)
macro_rules! Depcrate_constevalconst_eval_query {
() => {
// Module: crate::consteval
// Provides: {"const_eval_query"}
// Dependencies: {}
pub (crate) fn const_eval_query < 'db > (db : & 'db dyn HirDatabase , def : ConstId , subst : GenericArgs < 'db > , trait_env : Option < Arc < TraitEnvironment < 'db > > > ,) -> Result < Const < 'db > , ConstEvalError < 'db > > { let body = db . monomorphized_mir_body (def . into () , subst , db . trait_environment (def . into ())) ? ; let c = interpret_mir (db , body , false , trait_env) ? . 0 ? ; Ok (c) }
};
}
