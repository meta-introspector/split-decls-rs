// Generated macro for uwrite (macro)
macro_rules! Depcrate_sourceuwrite {
() => {
// Module: crate::source
// Provides: {"uwrite"}
// Dependencies: {}
# [doc = " Calls [`write!`] with the passed arguments and unwraps the result."] # [doc = ""] # [doc = " Useful for writing to things with infallible `Write` implementations like"] # [doc = " `Source` and `String`."] # [doc = ""] # [doc = " [`write!`]: std::write"] # [macro_export] macro_rules ! uwrite { ($ dst : expr , $ ($ arg : tt) *) => { write ! ($ dst , $ ($ arg) *) . unwrap () } ; }
};
}
