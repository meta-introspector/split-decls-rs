// Generated macro for deprecated_mach (macro)
macro_rules! Depcrate_macrosdeprecated_mach {
() => {
// Module: crate::macros
// Provides: {"deprecated_mach"}
// Dependencies: {}
macro_rules ! deprecated_mach { (pub const $ id : ident : $ ty : ty = $ expr : expr ;) => { # [deprecated (since = "0.2.55" , note = "Use the `mach2` crate instead" ,)] # [allow (deprecated)] pub const $ id : $ ty = $ expr ; } ; ($ (pub const $ id : ident : $ ty : ty = $ expr : expr ;) *) => { $ (deprecated_mach ! (pub const $ id : $ ty = $ expr ;) ;) * } ; (pub type $ id : ident = $ ty : ty ;) => { # [deprecated (since = "0.2.55" , note = "Use the `mach2` crate instead" ,)] # [allow (deprecated)] pub type $ id = $ ty ; } ; ($ (pub type $ id : ident = $ ty : ty ;) *) => { $ (deprecated_mach ! (pub type $ id = $ ty ;) ;) * } }
};
}
