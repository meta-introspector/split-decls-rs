// Generated macro for __pin_project_make_replace_field_proj (macro)
macro_rules! Depcrate__pin_project_make_replace_field_proj {
() => {
// Module: crate
// Provides: {"__pin_project_make_replace_field_proj"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __pin_project_make_replace_field_proj { (# [pin] $ field : ident) => { $ crate :: __private :: PhantomData } ; ($ field : ident) => { $ crate :: __private :: ptr :: read ($ field) } ; }
};
}
