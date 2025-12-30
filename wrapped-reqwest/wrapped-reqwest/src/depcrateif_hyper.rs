// Generated macro for if_hyper (macro)
macro_rules! Depcrateif_hyper {
() => {
// Module: crate
// Provides: {"if_hyper"}
// Dependencies: {}
macro_rules ! if_hyper { ($ ($ item : item) *) => { $ (# [cfg (not (target_arch = "wasm32"))] $ item) * } }
};
}
