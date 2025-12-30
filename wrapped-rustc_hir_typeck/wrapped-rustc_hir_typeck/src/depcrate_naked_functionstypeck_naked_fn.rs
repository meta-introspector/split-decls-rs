// Generated macro for typeck_naked_fn (function)
macro_rules! Depcrate_naked_functionstypeck_naked_fn {
() => {
// Module: crate::naked_functions
// Provides: {"typeck_naked_fn"}
// Dependencies: {}
# [doc = " Naked fns can only have trivial binding patterns in arguments,"] # [doc = " may not actually use those arguments, and the body must consist of just"] # [doc = " a single asm statement."] pub (crate) fn typeck_naked_fn < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId , body : & 'tcx hir :: Body < 'tcx > ,) { debug_assert ! (find_attr ! (tcx . get_all_attrs (def_id) , AttributeKind :: Naked (..))) ; check_no_patterns (tcx , body . params) ; check_no_parameters_use (tcx , body) ; check_asm (tcx , def_id , body) ; }
};
}
