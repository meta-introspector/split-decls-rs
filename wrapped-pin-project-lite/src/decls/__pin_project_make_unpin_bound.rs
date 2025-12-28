macro_rules! __pin_project_make_unpin_bound {
    () => {
        # [doc (hidden)] # [macro_export] macro_rules ! __pin_project_make_unpin_bound { (# [pin] $ field_ty : ty) => { $ field_ty } ; ($ field_ty : ty) => { $ crate :: __private :: AlwaysUnpin <$ field_ty > } ; }
    };
}

__pin_project_make_unpin_bound!()