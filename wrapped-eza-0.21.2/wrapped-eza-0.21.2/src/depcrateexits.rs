// Generated macro for exits (module)
macro_rules! Depcrateexits {
() => {
// Module: crate
// Provides: {"exits"}
// Dependencies: {}
mod exits { # [doc = " Exit code for when exa runs OK."] pub const SUCCESS : i32 = 0 ; # [doc = " Exit code for when there was at least one I/O error during execution."] pub const RUNTIME_ERROR : i32 = 1 ; # [doc = " Exit code for when the command-line options are invalid."] pub const OPTIONS_ERROR : i32 = 3 ; # [doc = " Exit code for missing file permissions"] pub const PERMISSION_DENIED : i32 = 13 ; }
};
}
