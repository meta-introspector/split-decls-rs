// Generated macro for __pin_project_make_unpin_bound (macro)
macro_rules! Depcrate__pin_project_make_unpin_bound {
() => {
// Module: crate
// Provides: {"__pin_project_make_unpin_bound"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __pin_project_make_unpin_bound { (# [pin] $ field_ty : ty) => { $ field_ty } ; ($ field_ty : ty) => { $ crate :: __private :: AlwaysUnpin <$ field_ty > } ; }
};
}
