macro_rules! __pin_project_make_unsafe_drop_in_place_guard {
    () => {
        # [doc (hidden)] # [macro_export] macro_rules ! __pin_project_make_unsafe_drop_in_place_guard { (# [pin] $ field : ident) => { $ crate :: __private :: UnsafeDropInPlaceGuard :: new ($ field) } ; ($ field : ident) => { () } ; }
    };
}

__pin_project_make_unsafe_drop_in_place_guard!()