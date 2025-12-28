macro_rules! __pin_project_struct_make_proj_replace_method {
    () => {
        # [doc (hidden)] # [macro_export] macro_rules ! __pin_project_struct_make_proj_replace_method { ([] $ ($ field : tt) *) => { } ; ([$ proj_ty_ident : ident] [$ proj_vis : vis] [$ _proj_ty_ident : ident] [$ ($ ty_generics : tt) *] { $ ($ (# [$ pin : ident]) ? $ field_vis : vis $ field : ident) ,+ }) => { # [doc (hidden)] # [inline] $ proj_vis fn project_replace (self : $ crate :: __private :: Pin <& mut Self >, replacement : Self ,) -> $ proj_ty_ident <$ ($ ty_generics) *> { unsafe { let __self_ptr : * mut Self = self . get_unchecked_mut () ; let __guard = $ crate :: __private :: UnsafeOverwriteGuard :: new (__self_ptr , replacement) ; let Self { $ ($ field) ,* } = & mut * __self_ptr ; $ crate :: __pin_project_make_proj_replace_block ! { [$ proj_ty_ident] { $ ($ (# [$ pin]) ? $ field) ,+ } } } } } ; }
    };
}

__pin_project_struct_make_proj_replace_method!()