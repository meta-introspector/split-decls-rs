// Generated macro for fold_quote (function)
macro_rules! Depcrate_utilsfold_quote {
() => {
// Module: crate::utils
// Provides: {"fold_quote"}
// Dependencies: {}
pub fn fold_quote < F , I , T > (input : impl Iterator < Item = I > , f : F) -> TokenStream where F : Fn (I) -> T , T : ToTokens , { input . fold (quote ! { } , | acc , x | { let y = f (x) ; quote ! { # acc # y } }) }
};
}
