macro_rules! __pin_project_make_proj_ty_body {
    () => {
        # [doc (hidden)] # [macro_export] macro_rules ! __pin_project_make_proj_ty_body { ([$ proj_ty_ident : ident] [$ proj_vis : vis $ struct_ty_ident : ident $ ident : ident] [$ ($ impl_generics : tt) *] [$ ($ ty_generics : tt) *] [$ (where $ ($ where_clause : tt) *) ?] [$ ($ body_data : tt) +]) => { # [doc (hidden)] # [allow (dead_code , single_use_lifetimes , clippy :: unknown_clippy_lints , clippy :: absolute_paths , clippy :: min_ident_chars , clippy :: mut_mut , clippy :: redundant_pub_crate , clippy :: ref_option_ref , clippy :: single_char_lifetime_names , clippy :: type_repetition_in_bounds)] $ proj_vis $ struct_ty_ident $ proj_ty_ident <'__pin , $ ($ impl_generics) *> where $ ident <$ ($ ty_generics) *>: '__pin $ (, $ ($ where_clause) *) ? { $ ($ body_data) + } } ; }
    };
}

__pin_project_make_proj_ty_body!()