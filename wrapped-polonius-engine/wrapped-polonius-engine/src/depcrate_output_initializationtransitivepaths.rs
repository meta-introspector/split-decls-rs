// Generated macro for TransitivePaths (struct)
macro_rules! Depcrate_output_initializationTransitivePaths {
() => {
// Module: crate::output::initialization
// Provides: {"TransitivePaths"}
// Dependencies: {}
struct TransitivePaths < T : FactTypes > { path_moved_at : Relation < (T :: Path , T :: Point) > , path_assigned_at : Relation < (T :: Path , T :: Point) > , path_accessed_at : Relation < (T :: Path , T :: Point) > , path_begins_with_var : Relation < (T :: Path , T :: Variable) > , }
};
}
