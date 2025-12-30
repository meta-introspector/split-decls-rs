// Generated macro for eq_precise_capture (function)
macro_rules! Depcrate_ast_utilseq_precise_capture {
() => {
// Module: crate::ast_utils
// Provides: {"eq_precise_capture"}
// Dependencies: {}
pub fn eq_precise_capture (l : & PreciseCapturingArg , r : & PreciseCapturingArg) -> bool { match (l , r) { (PreciseCapturingArg :: Lifetime (l) , PreciseCapturingArg :: Lifetime (r)) => l . ident == r . ident , (PreciseCapturingArg :: Arg (l , _) , PreciseCapturingArg :: Arg (r , _)) => l . segments [0] . ident == r . segments [0] . ident , _ => false , } }
};
}
