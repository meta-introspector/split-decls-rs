// Generated macro for __pin_project_enum_make_proj_replace_method (macro)
macro_rules! Depcrate__pin_project_enum_make_proj_replace_method {
() => {
// Module: crate
// Provides: {"__pin_project_enum_make_proj_replace_method"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __pin_project_enum_make_proj_replace_method { ([] $ ($ field : tt) *) => { } ; ([$ proj_ty_ident : ident] [$ proj_vis : vis] [$ ($ ty_generics : tt) *] { $ ($ variant : ident $ ({ $ ($ (# [$ pin : ident]) ? $ field : ident) ,+ }) ?) ,+ }) => { # [doc (hidden)] # [inline] $ proj_vis fn project_replace (self : $ crate :: __private :: Pin <& mut Self >, replacement : Self ,) -> $ proj_ty_ident <$ ($ ty_generics) *> { unsafe { let __self_ptr : * mut Self = self . get_unchecked_mut () ; let __guard = $ crate :: __private :: UnsafeOverwriteGuard :: new (__self_ptr , replacement) ; match & mut * __self_ptr { $ (Self ::$ variant $ ({ $ ($ field) ,+ }) ? => { $ crate :: __pin_project_make_proj_replace_block ! { [$ proj_ty_ident :: $ variant] $ ({ $ ($ (# [$ pin]) ? $ field) ,+ }) ? } }) ,+ } } } } ; }
};
}
