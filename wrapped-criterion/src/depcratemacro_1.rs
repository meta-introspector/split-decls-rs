// Generated macro for macro_1 (macro)
macro_rules! Depcratemacro_1 {
() => {
// Module: crate
// Provides: {"macro_1"}
// Dependencies: {}
# [cfg (all (feature = "rayon" , target_arch = "wasm32"))] compile_error ! ("Rayon cannot be used when targeting wasi32. Try disabling default features.") ;
};
}
