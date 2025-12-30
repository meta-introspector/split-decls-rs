// Generated macro for WORKING_DIRECTORY (static)
macro_rules! Depcrate_fsWORKING_DIRECTORY {
() => {
// Module: crate::fs
// Provides: {"WORKING_DIRECTORY"}
// Dependencies: {}
static WORKING_DIRECTORY : InterruptSpinMutex < Option < String > > = InterruptSpinMutex :: new (None) ;
};
}
