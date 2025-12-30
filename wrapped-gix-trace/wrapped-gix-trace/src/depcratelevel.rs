// Generated macro for Level (enum)
macro_rules! DepcrateLevel {
() => {
// Module: crate
// Provides: {"Level"}
// Dependencies: {}
# [doc = " The level at which the tracing item should be created."] # [doc = ""] # [doc = " It's used to filter items early."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Ord , PartialOrd)] pub enum Level { # [doc = " A coarse-grained trace level, one that should span entire operations with low frequency."] Coarse = 1 , # [doc = " Finer grained trace level that further subdivides coarse-level traces."] # [doc = ""] # [doc = " Note that these should only be created for areas of the code which have significant cost."] Detail = 2 , }
};
}
