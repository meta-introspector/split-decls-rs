macro_rules! __pin_project_enum_make_proj_method {
    () => {
        # [doc (hidden)] # [macro_export] macro_rules ! __pin_project_enum_make_proj_method { ([] $ ($ variant : tt) *) => { } ; ([$ proj_ty_ident : ident] [$ proj_vis : vis] [$ method_ident : ident $ get_method : ident $ ($ mut : ident) ?] [$ ($ ty_generics : tt) *] { $ ($ variant : ident $ ({ $ ($ (# [$ pin : ident]) ? $ field : ident) ,+ }) ?) ,+ }) => { # [doc (hidden)] # [inline] $ proj_vis fn $ method_ident <'__pin > (self : $ crate :: __private :: Pin <&'__pin $ ($ mut) ? Self >,) -> $ proj_ty_ident <'__pin , $ ($ ty_generics) *> { unsafe { match self .$ get_method () { $ (Self ::$ variant $ ({ $ ($ field) ,+ }) ? => { $ proj_ty_ident ::$ variant $ ({ $ ($ field : $ crate :: __pin_project_make_unsafe_field_proj ! ($ (# [$ pin]) ? $ field)) ,+ }) ? }) ,+ } } } } ; }
    };
}

__pin_project_enum_make_proj_method!()