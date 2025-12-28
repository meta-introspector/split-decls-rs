macro_rules! __pin_project_make_replace_field_proj {
    () => {
        # [doc (hidden)] # [macro_export] macro_rules ! __pin_project_make_replace_field_proj { (# [pin] $ field : ident) => { $ crate :: __private :: PhantomData } ; ($ field : ident) => { $ crate :: __private :: ptr :: read ($ field) } ; }
    };
}

__pin_project_make_replace_field_proj!()