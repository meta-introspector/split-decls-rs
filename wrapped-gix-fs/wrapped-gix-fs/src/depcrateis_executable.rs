// Generated macro for is_executable (function)
macro_rules! Depcrateis_executable {
() => {
// Module: crate
// Provides: {"is_executable"}
// Dependencies: {}
# [cfg (not (unix))] # [doc = " Returns whether a file has the executable permission set."] pub fn is_executable (_metadata : & std :: fs :: Metadata) -> bool { false }
};
}
