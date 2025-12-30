// Generated macro for compute (function)
macro_rules! Depcrate_output_initializationcompute {
() => {
// Module: crate::output::initialization
// Provides: {"compute"}
// Dependencies: {}
pub (super) fn compute < T : FactTypes > (ctx : InitializationContext < T > , cfg_edge : & Relation < (T :: Point , T :: Point) > , output : & mut Output < T > ,) -> InitializationResult < T > { let timer = Instant :: now () ; let transitive_paths = compute_transitive_paths :: < T > (ctx . child_path , ctx . path_assigned_at_base , ctx . path_moved_at_base , ctx . path_accessed_at_base , ctx . path_is_var ,) ; info ! ("initialization phase 1 completed: {:?}" , timer . elapsed ()) ; let InitializationStatus { var_maybe_partly_initialized_on_exit , move_error , } = compute_move_errors :: < T > (transitive_paths , cfg_edge , output) ; info ! ("initialization phase 2: {} move errors in {:?}" , move_error . elements . len () , timer . elapsed ()) ; if output . dump_enabled { for & (var , location) in var_maybe_partly_initialized_on_exit . iter () { output . var_maybe_partly_initialized_on_exit . entry (location) . or_default () . push (var) ; } } InitializationResult (var_maybe_partly_initialized_on_exit , move_error) }
};
}
