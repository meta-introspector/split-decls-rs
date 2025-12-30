// Generated macro for declare_features (macro)
macro_rules! Depcrate_removeddeclare_features {
() => {
// Module: crate::removed
// Provides: {"declare_features"}
// Dependencies: {}
macro_rules ! declare_features { ($ ($ (# [doc = $ doc : tt]) * (removed , $ feature : ident , $ ver : expr , $ issue : expr , $ reason : expr $ (, $ pull : expr) ?) ,) +) => { # [doc = " Formerly unstable features that have now been removed."] pub static REMOVED_LANG_FEATURES : & [RemovedFeature] = & [$ (RemovedFeature { feature : Feature { name : sym ::$ feature , since : $ ver , issue : to_nonzero ($ issue) , } , reason : $ reason , pull : opt_nonzero_u32 ! ($ ($ pull) ?) , }) ,+] ; } ; }
};
}
