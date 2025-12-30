// Generated macro for __pin_project_struct_make_proj_method (macro)
macro_rules! Depcrate__pin_project_struct_make_proj_method {
() => {
// Module: crate
// Provides: {"__pin_project_struct_make_proj_method"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __pin_project_struct_make_proj_method { ([] $ ($ variant : tt) *) => { } ; ([$ proj_ty_ident : ident $ _ignored_default_arg : ident] [$ proj_vis : vis] [$ method_ident : ident $ get_method : ident $ ($ mut : ident) ?] [$ ($ ty_generics : tt) *] $ ($ variant : tt) *) => { $ crate :: __pin_project_struct_make_proj_method ! { [$ proj_ty_ident] [$ proj_vis] [$ method_ident $ get_method $ ($ mut) ?] [$ ($ ty_generics) *] $ ($ variant) * } } ; ([$ proj_ty_ident : ident] [$ proj_vis : vis] [$ method_ident : ident $ get_method : ident $ ($ mut : ident) ?] [$ ($ ty_generics : tt) *] { $ ($ (# [$ pin : ident]) ? $ field_vis : vis $ field : ident) ,+ }) => { # [doc (hidden)] # [inline] $ proj_vis fn $ method_ident <'__pin > (self : $ crate :: __private :: Pin <&'__pin $ ($ mut) ? Self >,) -> $ proj_ty_ident <'__pin , $ ($ ty_generics) *> { unsafe { let Self { $ ($ field) ,* } = self .$ get_method () ; $ proj_ty_ident { $ ($ field : $ crate :: __pin_project_make_unsafe_field_proj ! ($ (# [$ pin]) ? $ field)) ,+ } } } } ; }
};
}
