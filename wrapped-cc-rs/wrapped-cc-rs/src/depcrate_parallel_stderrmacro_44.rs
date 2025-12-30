// Generated macro for macro_44 (macro)
macro_rules! Depcrate_parallel_stderrmacro_44 {
() => {
// Module: crate::parallel::stderr
// Provides: {"macro_44"}
// Dependencies: {}
# [cfg (all (not (unix) , not (windows) , not (target_family = "wasm")))] compile_error ! ("Only unix and windows support non-blocking pipes! For other OSes, disable the parallel feature.") ;
};
}
