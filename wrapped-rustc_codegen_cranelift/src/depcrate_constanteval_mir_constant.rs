// Generated macro for eval_mir_constant (function)
macro_rules! Depcrate_constanteval_mir_constant {
() => {
// Module: crate::constant
// Provides: {"eval_mir_constant"}
// Dependencies: {}
pub (crate) fn eval_mir_constant < 'tcx > (fx : & FunctionCx < '_ , '_ , 'tcx > , constant : & ConstOperand < 'tcx > ,) -> (ConstValue , Ty < 'tcx >) { let cv = fx . monomorphize (constant . const_) ; let val = cv . eval (fx . tcx , ty :: TypingEnv :: fully_monomorphized () , constant . span) . expect ("erroneous constant missed by mono item collection") ; (val , cv . ty ()) }
};
}
