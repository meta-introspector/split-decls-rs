// Generated macro for set_default_instrumentation (function)
macro_rules! Depcrate_connection_instrumentationset_default_instrumentation {
() => {
// Module: crate::connection::instrumentation
// Provides: {"set_default_instrumentation"}
// Dependencies: {}
# [doc = " Set a custom constructor for the default [`Instrumentation`]"] # [doc = " used by new connections"] # [doc = ""] # [doc = " ```rust"] # [doc = " use diesel::connection::{set_default_instrumentation, Instrumentation, InstrumentationEvent};"] # [doc = ""] # [doc = " // a simple logger that prints all events to stdout"] # [doc = " fn simple_logger() -> Option<Box<dyn Instrumentation>> {"] # [doc = "     // we need the explicit argument type there due"] # [doc = "     // to bugs in rustc"] # [doc = "     Some(Box::new(|event: InstrumentationEvent<'_>| {"] # [doc = "         println!(\"{event:?}\")"] # [doc = "     }))"] # [doc = " }"] # [doc = ""] # [doc = " set_default_instrumentation(simple_logger);"] # [doc = " ```"] pub fn set_default_instrumentation (default : fn () -> Option < Box < dyn Instrumentation > > ,) -> crate :: QueryResult < () > { match GLOBAL_INSTRUMENTATION . write () { Ok (mut l) => { * l = default ; Ok (()) } Err (e) => Err (crate :: result :: Error :: DatabaseError (crate :: result :: DatabaseErrorKind :: Unknown , Box :: new (e . to_string ()) ,)) , } }
};
}
