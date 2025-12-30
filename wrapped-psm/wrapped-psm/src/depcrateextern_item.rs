// Generated macro for extern_item (macro)
macro_rules! Depcrateextern_item {
() => {
// Module: crate
// Provides: {"extern_item"}
// Dependencies: {}
# [cfg (target_arch = "arm")] macro_rules ! extern_item { (unsafe $ ($ toks : tt) +) => { unsafe extern "aapcs" $ ($ toks) + } ; ($ ($ toks : tt) +) => { extern "aapcs" $ ($ toks) + } ; }
};
}
