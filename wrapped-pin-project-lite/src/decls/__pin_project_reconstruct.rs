macro_rules! __pin_project_reconstruct {
    () => {
        # [doc (hidden)] # [macro_export] macro_rules ! __pin_project_reconstruct { ([$ (# [$ attrs : meta]) * $ vis : vis struct $ ident : ident] [$ ($ def_generics : tt) *] [$ ($ impl_generics : tt) *] [$ ($ ty_generics : tt) *] [$ (where $ ($ where_clause : tt) *) ?] { $ ($ (# [$ pin : ident]) ? $ field_vis : vis $ field : ident : $ field_ty : ty) ,+ $ (,) ? }) => { $ (# [$ attrs]) * $ vis struct $ ident $ ($ def_generics) * $ (where $ ($ where_clause) *) ? { $ ($ field_vis $ field : $ field_ty) ,+ } } ; ([$ (# [$ attrs : meta]) * $ vis : vis enum $ ident : ident] [$ ($ def_generics : tt) *] [$ ($ impl_generics : tt) *] [$ ($ ty_generics : tt) *] [$ (where $ ($ where_clause : tt) *) ?] { $ ($ (# [$ variant_attrs : meta]) * $ variant : ident $ ({ $ ($ (# [$ pin : ident]) ? $ field : ident : $ field_ty : ty) ,+ $ (,) ? }) ?) ,+ $ (,) ? }) => { $ (# [$ attrs]) * $ vis enum $ ident $ ($ def_generics) * $ (where $ ($ where_clause) *) ? { $ ($ (# [$ variant_attrs]) * $ variant $ ({ $ ($ field : $ field_ty) ,+ }) ?) ,+ } } ; }
    };
}

__pin_project_reconstruct!();