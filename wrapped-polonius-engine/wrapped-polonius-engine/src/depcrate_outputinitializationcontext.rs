// Generated macro for InitializationContext (struct)
macro_rules! Depcrate_outputInitializationContext {
() => {
// Module: crate::output
// Provides: {"InitializationContext"}
// Dependencies: {}
# [doc = " Subset of `AllFacts` dedicated to initialization"] struct InitializationContext < T : FactTypes > { child_path : Vec < (T :: Path , T :: Path) > , path_is_var : Vec < (T :: Path , T :: Variable) > , path_assigned_at_base : Vec < (T :: Path , T :: Point) > , path_moved_at_base : Vec < (T :: Path , T :: Point) > , path_accessed_at_base : Vec < (T :: Path , T :: Point) > , }
};
}
