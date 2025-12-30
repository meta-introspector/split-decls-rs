// Generated macro for emit_error (macro)
macro_rules! Depcrate_macrosemit_error {
() => {
// Module: crate::macros
// Provides: {"emit_error"}
// Dependencies: {}
# [doc = " Emit an error while not aborting the proc-macro right away."] # [doc = ""] # [doc = " # Syntax"] # [doc = ""] # [doc = " See [the guide](index.html#guide)."] # [doc = ""] # [macro_export] macro_rules ! emit_error { ($ err : expr) => { $ crate :: diagnostic ! ($ err) . emit () } ; ($ span : expr , $ ($ tts : tt) *) => { { let level = $ crate :: Level :: Error ; $ crate :: diagnostic ! ($ span , level , $ ($ tts) *) . emit () } } ; }
};
}
