// Generated macro for __pin_project_make_proj_field_ref (macro)
macro_rules! Depcrate__pin_project_make_proj_field_ref {
() => {
// Module: crate
// Provides: {"__pin_project_make_proj_field_ref"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __pin_project_make_proj_field_ref { (# [pin] $ field_ty : ty) => { $ crate :: __private :: Pin <&'__pin ($ field_ty) > } ; ($ field_ty : ty) => { &'__pin ($ field_ty) } ; }
};
}
