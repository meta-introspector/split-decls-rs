macro_rules! __pin_project_make_proj_field_mut {
    () => {
        # [doc (hidden)] # [macro_export] macro_rules ! __pin_project_make_proj_field_mut { (# [pin] $ field_ty : ty) => { $ crate :: __private :: Pin <&'__pin mut ($ field_ty) > } ; ($ field_ty : ty) => { &'__pin mut ($ field_ty) } ; }
    };
}

__pin_project_make_proj_field_mut!();