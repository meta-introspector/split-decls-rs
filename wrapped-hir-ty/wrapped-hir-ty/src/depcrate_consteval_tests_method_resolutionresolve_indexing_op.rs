// Generated macro for resolve_indexing_op (function)
macro_rules! Depcrate_consteval_tests_method_resolutionresolve_indexing_op {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"resolve_indexing_op"}
// Dependencies: {}
# [doc = " Returns the receiver type for the index trait call."] pub (crate) fn resolve_indexing_op (db : & dyn HirDatabase , env : Arc < TraitEnvironment > , ty : Canonical < Ty > , index_trait : TraitId ,) -> Option < ReceiverAdjustments > { let mut table = InferenceTable :: new (db , env) ; let ty = table . instantiate_canonical (ty) ; let deref_chain = autoderef_method_receiver (& mut table , ty) ; for (ty , adj) in deref_chain { let goal = generic_implements_goal (db , & table . trait_env , index_trait , & ty) ; if db . trait_solve (table . trait_env . krate , table . trait_env . block , goal . cast (Interner)) . is_some () { return Some (adj) ; } } None }
};
}
