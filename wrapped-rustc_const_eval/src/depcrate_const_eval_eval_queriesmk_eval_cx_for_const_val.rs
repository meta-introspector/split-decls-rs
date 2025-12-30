// Generated macro for mk_eval_cx_for_const_val (function)
macro_rules! Depcrate_const_eval_eval_queriesmk_eval_cx_for_const_val {
() => {
// Module: crate::const_eval::eval_queries
// Provides: {"mk_eval_cx_for_const_val"}
// Dependencies: {}
# [doc = " Create an interpreter context to inspect the given `ConstValue`."] # [doc = " Returns both the context and an `OpTy` that represents the constant."] pub fn mk_eval_cx_for_const_val < 'tcx > (tcx : TyCtxtAt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , val : mir :: ConstValue , ty : Ty < 'tcx > ,) -> Option < (CompileTimeInterpCx < 'tcx > , OpTy < 'tcx >) > { let ecx = mk_eval_cx_to_read_const_val (tcx . tcx , tcx . span , typing_env , CanAccessMutGlobal :: No) ; let op = ecx . const_val_to_op (val , ty , None) . discard_err () ? ; Some ((ecx , op)) }
};
}
