macro_rules! __pin_project_make_proj_field_replace {
    () => {
        # [doc (hidden)] # [macro_export] macro_rules ! __pin_project_make_proj_field_replace { (# [pin] $ field_ty : ty) => { $ crate :: __private :: PhantomData <$ field_ty > } ; ($ field_ty : ty) => { $ field_ty } ; }
    };
}

__pin_project_make_proj_field_replace!()