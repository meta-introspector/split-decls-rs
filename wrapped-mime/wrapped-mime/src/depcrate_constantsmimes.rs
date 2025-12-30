// Generated macro for mimes (macro)
macro_rules! Depcrate_constantsmimes {
() => {
// Module: crate::constants
// Provides: {"mimes"}
// Dependencies: {}
macro_rules ! mimes { ($ (@ $ kind : ident : $ ($ id : ident , $ src : expr ;) +) +) => (pub (super) mod mimes { use crate :: { MediaType , MediaRange } ; $ ($ (mime_constant ! { $ kind , $ id , $ src }) +) + } # [test] fn test_mimes_macro_consts () { use self :: mimes ::*; $ ($ (mime_constant_test ! { $ id , $ src }) +) + $ ($ (mime_constant_proc_macro_test ! { @$ kind , $ id , $ src }) +) + }) }
};
}
