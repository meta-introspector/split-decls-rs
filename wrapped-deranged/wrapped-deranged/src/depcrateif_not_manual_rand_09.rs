// Generated macro for if_not_manual_rand_09 (macro)
macro_rules! Depcrateif_not_manual_rand_09 {
() => {
// Module: crate
// Provides: {"if_not_manual_rand_09"}
// Dependencies: {}
# [doc = " Output the provided code if and only if the list does not include `rand_09`."] # [allow (unused_macro_rules)] macro_rules ! if_not_manual_rand_09 { ([rand_09 $ ($ rest : ident) *] $ ($ output : tt) *) => { } ; ([] $ ($ output : tt) *) => { $ ($ output) * } ; ([$ first : ident $ ($ rest : ident) *] $ ($ output : tt) *) => { if_not_manual_rand_09 ! ([$ ($ rest) *] $ ($ output) *) ; } ; }
};
}
