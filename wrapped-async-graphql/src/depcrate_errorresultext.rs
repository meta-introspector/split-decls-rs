// Generated macro for ResultExt (trait)
macro_rules! Depcrate_errorResultExt {
() => {
// Module: crate::error
// Provides: {"ResultExt"}
// Dependencies: {}
# [doc = " Extend a `Result`'s error value with"] # [doc = " [`ErrorExtensions`](trait.ErrorExtensions.html)."] pub trait ResultExt < T , E > : Sized { # [doc = " Extend the error value of the result with the callback."] fn extend_err < C > (self , cb : C) -> Result < T > where C : FnOnce (& E , & mut ErrorExtensionValues) ; # [doc = " Extend the result to a `Result`."] fn extend (self) -> Result < T > ; }
};
}
