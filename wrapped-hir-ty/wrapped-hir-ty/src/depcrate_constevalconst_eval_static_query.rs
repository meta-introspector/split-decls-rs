// Generated macro for const_eval_static_query (function)
macro_rules! Depcrate_constevalconst_eval_static_query {
() => {
// Module: crate::consteval
// Provides: {"const_eval_static_query"}
// Dependencies: {}
pub (crate) fn const_eval_static_query < 'db > (db : & 'db dyn HirDatabase , def : StaticId ,) -> Result < Const < 'db > , ConstEvalError < 'db > > { let interner = DbInterner :: new_with (db , None , None) ; let body = db . monomorphized_mir_body (def . into () , GenericArgs :: new_from_iter (interner , []) , db . trait_environment_for_body (def . into ()) ,) ? ; let c = interpret_mir (db , body , false , None) ? . 0 ? ; Ok (c) }
};
}
