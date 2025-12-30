// Generated macro for emit_warning (macro)
macro_rules! Depcrate_macrosemit_warning {
() => {
// Module: crate::macros
// Provides: {"emit_warning"}
// Dependencies: {}
# [doc = " Emit a warning. Warnings are not errors and compilation won't fail because of them."] # [doc = ""] # [doc = " **Does nothing on stable**"] # [doc = ""] # [doc = " # Syntax"] # [doc = ""] # [doc = " See [the guide](index.html#guide)."] # [doc = ""] # [macro_export] macro_rules ! emit_warning { ($ span : expr , $ ($ tts : tt) *) => { $ crate :: diagnostic ! ($ span , $ crate :: Level :: Warning , $ ($ tts) *) . emit () } ; }
};
}
