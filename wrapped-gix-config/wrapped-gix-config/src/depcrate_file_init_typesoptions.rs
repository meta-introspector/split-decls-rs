// Generated macro for Options (struct)
macro_rules! Depcrate_file_init_typesOptions {
() => {
// Module: crate::file::init::types
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options when loading git config using [`File::from_paths_metadata()`][crate::File::from_paths_metadata()]."] # [derive (Clone , Copy , Default)] pub struct Options < 'a > { # [doc = " Configure how to follow includes while handling paths."] pub includes : init :: includes :: Options < 'a > , # [doc = " If true, only value-bearing parse events will be kept to reduce memory usage and increase performance."] # [doc = ""] # [doc = " Note that doing so will degenerate [`write_to()`][crate::File::write_to()] and strip it off its comments"] # [doc = " and additional whitespace entirely, but will otherwise be a valid configuration file."] pub lossy : bool , # [doc = " If true, any IO error happening when reading a configuration file will be ignored."] # [doc = ""] # [doc = " That way it's possible to pass multiple files and read as many as possible, to have 'something' instead of nothing."] pub ignore_io_errors : bool , }
};
}
