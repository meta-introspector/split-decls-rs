// Generated macro for is_done_windows (function)
macro_rules! Depcrate_pathis_done_windows {
() => {
// Module: crate::path
// Provides: {"is_done_windows"}
// Dependencies: {}
# [doc = " Check if trailing filename bytes leave a match to Windows reserved device names unchanged."] fn is_done_windows (input : Option < & [u8] >) -> bool { let Some (input) = input else { return true } ; let skip = input . bytes () . take_while (| b | * b == b' ') . count () ; let Some (next) = input . get (skip) else { return true } ; * next == b'.' || * next == b':' }
};
}
