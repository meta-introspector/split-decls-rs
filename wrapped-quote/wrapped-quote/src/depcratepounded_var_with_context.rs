// Generated macro for pounded_var_with_context (macro)
macro_rules! Depcratepounded_var_with_context {
() => {
// Module: crate
// Provides: {"pounded_var_with_context"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! pounded_var_with_context { ($ call : ident ! $ extra : tt $ b1 : tt ($ ($ inner : tt) *)) => { $ crate :: pounded_var_names ! { $ call ! $ extra $ ($ inner) * } } ; ($ call : ident ! $ extra : tt $ b1 : tt [$ ($ inner : tt) *]) => { $ crate :: pounded_var_names ! { $ call ! $ extra $ ($ inner) * } } ; ($ call : ident ! $ extra : tt $ b1 : tt { $ ($ inner : tt) * }) => { $ crate :: pounded_var_names ! { $ call ! $ extra $ ($ inner) * } } ; ($ call : ident ! ($ ($ extra : tt) *) # $ var : ident) => { $ crate ::$ call ! ($ ($ extra) * $ var) ; } ; ($ call : ident ! $ extra : tt $ b1 : tt $ curr : tt) => { } ; }
};
}
