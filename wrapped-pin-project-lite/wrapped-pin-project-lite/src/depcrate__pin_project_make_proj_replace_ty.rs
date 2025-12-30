// Generated macro for __pin_project_make_proj_replace_ty (macro)
macro_rules! Depcrate__pin_project_make_proj_replace_ty {
() => {
// Module: crate
// Provides: {"__pin_project_make_proj_replace_ty"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __pin_project_make_proj_replace_ty { ([] $ ($ field : tt) *) => { } ; ([$ proj_ty_ident : ident] [$ proj_vis : vis struct] [$ __pin_project_make_proj_field : ident] [$ ($ impl_generics : tt) *] [$ ($ ty_generics : tt) *] [$ (where $ ($ where_clause : tt) *) ?] { $ ($ (# [$ pin : ident]) ? $ field_vis : vis $ field : ident : $ field_ty : ty) ,+ $ (,) ? }) => { $ crate :: __pin_project_make_proj_replace_ty_body ! { [$ proj_ty_ident] [$ proj_vis struct] [$ ($ impl_generics) *] [$ ($ ty_generics) *] [$ (where $ ($ where_clause) *) ?] [$ ($ field_vis $ field : $ crate ::$ __pin_project_make_proj_field ! ($ (# [$ pin]) ? $ field_ty)) ,+] } } ; ([$ proj_ty_ident : ident] [$ proj_vis : vis enum] [$ __pin_project_make_proj_field : ident] [$ ($ impl_generics : tt) *] [$ ($ ty_generics : tt) *] [$ (where $ ($ where_clause : tt) *) ?] { $ ($ (# [$ variant_attrs : meta]) * $ variant : ident $ ({ $ ($ (# [$ pin : ident]) ? $ field : ident : $ field_ty : ty) ,+ $ (,) ? }) ?) ,+ $ (,) ? }) => { $ crate :: __pin_project_make_proj_replace_ty_body ! { [$ proj_ty_ident] [$ proj_vis enum] [$ ($ impl_generics) *] [$ ($ ty_generics) *] [$ (where $ ($ where_clause) *) ?] [$ ($ variant $ ({ $ ($ field : $ crate ::$ __pin_project_make_proj_field ! ($ (# [$ pin]) ? $ field_ty)) ,+ }) ?) ,+] } } ; }
};
}
