// Generated macro for ConstEvalCtxt (struct)
macro_rules! Depcrate_constsConstEvalCtxt {
() => {
// Module: crate::consts
// Provides: {"ConstEvalCtxt"}
// Dependencies: {}
# [doc = " The context required to evaluate a constant expression."] # [doc = ""] # [doc = " This is currently limited to constant folding and reading the value of named constants."] # [doc = ""] # [doc = " See the module level documentation for some context."] pub struct ConstEvalCtxt < 'tcx > { tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , typeck : & 'tcx TypeckResults < 'tcx > , source : Cell < ConstantSource > , ctxt : Cell < SyntaxContext > , }
};
}
