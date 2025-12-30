// Generated macro for InterpCx (struct)
macro_rules! Depcrate_interpret_eval_contextInterpCx {
() => {
// Module: crate::interpret::eval_context
// Provides: {"InterpCx"}
// Dependencies: {}
pub struct InterpCx < 'tcx , M : Machine < 'tcx > > { # [doc = " Stores the `Machine` instance."] # [doc = ""] # [doc = " Note: the stack is provided by the machine."] pub machine : M , # [doc = " The results of the type checker, from rustc."] # [doc = " The span in this is the \"root\" of the evaluation, i.e., the const"] # [doc = " we are evaluating (if this is CTFE)."] pub tcx : TyCtxtAt < 'tcx > , # [doc = " The current context in case we're evaluating in a"] # [doc = " polymorphic context. This always uses `ty::TypingMode::PostAnalysis`."] pub (super) typing_env : ty :: TypingEnv < 'tcx > , # [doc = " The virtual memory system."] pub memory : Memory < 'tcx , M > , # [doc = " The recursion limit (cached from `tcx.recursion_limit(())`)"] pub recursion_limit : Limit , }
};
}
