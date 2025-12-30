// Generated macro for macro_1 (macro)
macro_rules! Depcratemacro_1 {
() => {
// Module: crate
// Provides: {"macro_1"}
// Dependencies: {}
# [cfg (not (any (target_pointer_width = "32" , target_pointer_width = "64")))] compile_error ! ("jiff currently not supported on non-{32,64}") ;
};
}
