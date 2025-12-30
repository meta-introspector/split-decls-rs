// Generated macro for Instrumentation (trait)
macro_rules! Depcrate_connection_instrumentationInstrumentation {
() => {
// Module: crate::connection::instrumentation
// Provides: {"Instrumentation"}
// Dependencies: {}
# [doc = " A type that provides an connection `Instrumentation`"] # [doc = ""] # [doc = " This trait is the basic building block for logging or"] # [doc = " otherwise instrumenting diesel connection types. It"] # [doc = " acts as callback that receives information about certain"] # [doc = " important connection states"] # [doc = ""] # [doc = " For simple usages this trait is implemented for closures"] # [doc = " accepting a [`InstrumentationEvent`] as argument."] # [doc = ""] # [doc = " More complex usages and integrations with frameworks like"] # [doc = " `tracing` and `log` are supposed to be part of their own"] # [doc = " crates."] pub trait Instrumentation : Send + 'static { # [doc = " The function that is invoked for each event"] fn on_connection_event (& mut self , event : InstrumentationEvent < '_ >) ; }
};
}
