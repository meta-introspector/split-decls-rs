// Generated macro for macro_1 (macro)
macro_rules! Depcratemacro_1 {
() => {
// Module: crate
// Provides: {"macro_1"}
// Dependencies: {}
# [cfg (all (feature = "std" , feature = "const-random"))] compile_error ! ("default feature 'std' and feature 'const-random' cannot be enabled at the same time: for no_std environments, disable 'std' and enable 'const-random'") ;
};
}
