// Generated macro for is_executable (function)
macro_rules! Depcrateis_executable {
() => {
// Module: crate
// Provides: {"is_executable"}
// Dependencies: {}
# [cfg (windows)] fn is_executable < P : AsRef < Path > > (path : P) -> bool { path . as_ref () . is_file () }
};
}
