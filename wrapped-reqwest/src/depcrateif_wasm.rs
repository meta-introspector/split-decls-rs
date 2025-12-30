// Generated macro for if_wasm (macro)
macro_rules! Depcrateif_wasm {
() => {
// Module: crate
// Provides: {"if_wasm"}
// Dependencies: {}
macro_rules ! if_wasm { ($ ($ item : item) *) => { $ (# [cfg (target_arch = "wasm32")] $ item) * } }
};
}
