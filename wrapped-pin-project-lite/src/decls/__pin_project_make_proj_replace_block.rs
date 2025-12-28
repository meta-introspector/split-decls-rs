macro_rules! __pin_project_make_proj_replace_block {
    () => {
        # [doc (hidden)] # [macro_export] macro_rules ! __pin_project_make_proj_replace_block { ([$ ($ proj_path : tt) +] { $ ($ (# [$ pin : ident]) ? $ field_vis : vis $ field : ident) ,+ }) => { let result = $ ($ proj_path) * { $ ($ field : $ crate :: __pin_project_make_replace_field_proj ! ($ (# [$ pin]) ? $ field)) ,+ } ; { ($ ($ crate :: __pin_project_make_unsafe_drop_in_place_guard ! ($ (# [$ pin]) ? $ field) ,) *) ; } result } ; ([$ ($ proj_path : tt) +]) => { $ ($ proj_path) * } ; }
    };
}

__pin_project_make_proj_replace_block!()