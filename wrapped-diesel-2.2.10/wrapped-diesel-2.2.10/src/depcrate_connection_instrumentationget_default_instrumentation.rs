// Generated macro for get_default_instrumentation (function)
macro_rules! Depcrate_connection_instrumentationget_default_instrumentation {
() => {
// Module: crate::connection::instrumentation
// Provides: {"get_default_instrumentation"}
// Dependencies: {}
# [doc = " Get an instance of the default [`Instrumentation`]"] # [doc = ""] # [doc = " This function is mostly useful for crates implementing"] # [doc = " their own connection types"] pub fn get_default_instrumentation () -> Option < Box < dyn Instrumentation > > { match GLOBAL_INSTRUMENTATION . read () { Ok (f) => (* f) () , Err (_) => None , } }
};
}
