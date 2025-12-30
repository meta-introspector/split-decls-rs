// Generated macro for uwriteln (macro)
macro_rules! Depcrate_sourceuwriteln {
() => {
// Module: crate::source
// Provides: {"uwriteln"}
// Dependencies: {}
# [doc = " Calls [`writeln!`] with the passed arguments and unwraps the result."] # [doc = ""] # [doc = " Useful for writing to things with infallible `Write` implementations like"] # [doc = " `Source` and `String`."] # [doc = ""] # [doc = " [`writeln!`]: std::writeln"] # [macro_export] macro_rules ! uwriteln { ($ dst : expr , $ ($ arg : tt) *) => { writeln ! ($ dst , $ ($ arg) *) . unwrap () } ; }
};
}
