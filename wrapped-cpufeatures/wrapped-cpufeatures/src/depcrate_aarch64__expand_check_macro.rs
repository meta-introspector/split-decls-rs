// Generated macro for __expand_check_macro (macro)
macro_rules! Depcrate_aarch64__expand_check_macro {
() => {
// Module: crate::aarch64
// Provides: {"__expand_check_macro"}
// Dependencies: {}
# [cfg (any (target_os = "linux" , target_os = "android"))] macro_rules ! __expand_check_macro { ($ (($ name : tt , $ hwcap : ident)) ,* $ (,) ?) => { # [macro_export] # [doc (hidden)] macro_rules ! check { $ (($ hwcaps : expr , $ name) => { (($ hwcaps & $ crate :: aarch64 :: hwcaps ::$ hwcap) != 0) } ;) * } } ; }
};
}
