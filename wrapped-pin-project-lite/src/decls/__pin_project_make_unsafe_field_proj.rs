macro_rules! __pin_project_make_unsafe_field_proj {
    () => {
        # [doc (hidden)] # [macro_export] macro_rules ! __pin_project_make_unsafe_field_proj { (# [pin] $ field : ident) => { $ crate :: __private :: Pin :: new_unchecked ($ field) } ; ($ field : ident) => { $ field } ; }
    };
}

__pin_project_make_unsafe_field_proj!()