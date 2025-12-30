// Generated macro for is_executable (function)
macro_rules! Depcrate_supportis_executable {
() => {
// Module: crate::support
// Provides: {"is_executable"}
// Dependencies: {}
# [cfg (not (unix))] fn is_executable (_ : & Path) -> io :: Result < bool > { Ok (true) }
};
}
