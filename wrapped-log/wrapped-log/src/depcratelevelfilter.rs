// Generated macro for LevelFilter (enum)
macro_rules! DepcrateLevelFilter {
() => {
// Module: crate
// Provides: {"LevelFilter"}
// Dependencies: {}
# [doc = " An enum representing the available verbosity level filters of the logger."] # [doc = ""] # [doc = " A `LevelFilter` may be compared directly to a [`Level`]. Use this type"] # [doc = " to get and set the maximum log level with [`max_level()`] and [`set_max_level`]."] # [doc = ""] # [doc = " [`Level`]: enum.Level.html"] # [doc = " [`max_level()`]: fn.max_level.html"] # [doc = " [`set_max_level`]: fn.set_max_level.html"] # [repr (usize)] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Debug , Hash)] pub enum LevelFilter { # [doc = " A level lower than all log levels."] Off , # [doc = " Corresponds to the `Error` log level."] Error , # [doc = " Corresponds to the `Warn` log level."] Warn , # [doc = " Corresponds to the `Info` log level."] Info , # [doc = " Corresponds to the `Debug` log level."] Debug , # [doc = " Corresponds to the `Trace` log level."] Trace , }
};
}
