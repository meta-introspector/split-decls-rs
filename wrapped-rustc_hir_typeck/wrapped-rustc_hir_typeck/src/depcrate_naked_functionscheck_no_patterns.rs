// Generated macro for check_no_patterns (function)
macro_rules! Depcrate_naked_functionscheck_no_patterns {
() => {
// Module: crate::naked_functions
// Provides: {"check_no_patterns"}
// Dependencies: {}
# [doc = " Checks that parameters don't use patterns. Mirrors the checks for function declarations."] fn check_no_patterns (tcx : TyCtxt < '_ > , params : & [hir :: Param < '_ >]) { for param in params { match param . pat . kind { hir :: PatKind :: Wild | hir :: PatKind :: Binding (hir :: BindingMode :: NONE , _ , _ , None) => { } _ => { tcx . dcx () . emit_err (NoPatterns { span : param . pat . span }) ; } } } }
};
}
