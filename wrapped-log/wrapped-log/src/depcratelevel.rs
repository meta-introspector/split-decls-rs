// Generated macro for Level (enum)
macro_rules! DepcrateLevel {
() => {
// Module: crate
// Provides: {"Level"}
// Dependencies: {}
# [doc = " An enum representing the available verbosity levels of the logger."] # [doc = ""] # [doc = " Typical usage includes: checking if a certain `Level` is enabled with"] # [doc = " [`log_enabled!`](macro.log_enabled.html), specifying the `Level` of"] # [doc = " [`log!`](macro.log.html), and comparing a `Level` directly to a"] # [doc = " [`LevelFilter`](enum.LevelFilter.html)."] # [repr (usize)] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Debug , Hash)] pub enum Level { # [doc = " The \"error\" level."] # [doc = ""] # [doc = " Designates very serious errors."] Error = 1 , # [doc = " The \"warn\" level."] # [doc = ""] # [doc = " Designates hazardous situations."] Warn , # [doc = " The \"info\" level."] # [doc = ""] # [doc = " Designates useful information."] Info , # [doc = " The \"debug\" level."] # [doc = ""] # [doc = " Designates lower priority information."] Debug , # [doc = " The \"trace\" level."] # [doc = ""] # [doc = " Designates very low priority, often extremely verbose, information."] Trace , }
};
}
