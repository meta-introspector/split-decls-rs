// Generated macro for event (module)
macro_rules! Depcrateevent {
() => {
// Module: crate
// Provides: {"event"}
// Dependencies: {}
# [doc = ""] pub mod event { # [cfg (feature = "tracing")] pub use tracing_core :: Level ; # [doc = " All available tracing levels for use in `event!()` macro."] # [cfg (not (feature = "tracing"))] # [repr (usize)] # [derive (Copy , Clone , Debug , Hash , Eq , PartialEq)] pub enum Level { # [doc = " The \"trace\" level."] # [doc = ""] # [doc = " Designates very low priority, often extremely verbose, information."] TRACE = 0 , # [doc = " The \"debug\" level."] # [doc = ""] # [doc = " Designates lower priority information."] DEBUG = 1 , # [doc = " The \"info\" level."] # [doc = ""] # [doc = " Designates useful information."] INFO = 2 , # [doc = " The \"warn\" level."] # [doc = ""] # [doc = " Designates hazardous situations."] WARN = 3 , # [doc = " The \"error\" level."] # [doc = ""] # [doc = " Designates very serious errors."] ERROR = 4 , } }
};
}
