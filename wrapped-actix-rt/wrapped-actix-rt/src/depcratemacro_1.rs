// Generated macro for macro_1 (macro)
macro_rules! Depcratemacro_1 {
() => {
// Module: crate
// Provides: {"macro_1"}
// Dependencies: {}
# [cfg (all (not (target_os = "linux") , feature = "io-uring"))] compile_error ! ("io_uring is a linux only feature.") ;
};
}
