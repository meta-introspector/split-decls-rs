// Generated macro for pounded_var_names (macro)
macro_rules! Depcrate_tokenspounded_var_names {
() => {
// Module: crate::tokens
// Provides: {"pounded_var_names"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! pounded_var_names { ($ call : ident ! $ extra : tt $ ($ tts : tt) *) => { $ crate :: pounded_var_names_with_context ! ($ call ! $ extra (@ $ ($ tts) *) ($ ($ tts) * @)) } ; }
};
}
