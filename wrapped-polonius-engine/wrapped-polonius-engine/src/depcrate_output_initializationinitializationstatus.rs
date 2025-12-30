// Generated macro for InitializationStatus (struct)
macro_rules! Depcrate_output_initializationInitializationStatus {
() => {
// Module: crate::output::initialization
// Provides: {"InitializationStatus"}
// Dependencies: {}
struct InitializationStatus < T : FactTypes > { var_maybe_partly_initialized_on_exit : Relation < (T :: Variable , T :: Point) > , move_error : Relation < (T :: Path , T :: Point) > , }
};
}
