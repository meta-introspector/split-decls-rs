// Generated macro for declare_features (macro)
macro_rules! Depcrate_accepteddeclare_features {
() => {
// Module: crate::accepted
// Provides: {"declare_features"}
// Dependencies: {}
macro_rules ! declare_features { ($ ($ (# [doc = $ doc : tt]) * (accepted , $ feature : ident , $ ver : expr , $ issue : expr) ,) +) => { # [doc = " Formerly unstable features that have now been accepted (stabilized)."] pub static ACCEPTED_LANG_FEATURES : & [Feature] = & [$ (Feature { name : sym ::$ feature , since : $ ver , issue : to_nonzero ($ issue) , }) ,+] ; } }
};
}
