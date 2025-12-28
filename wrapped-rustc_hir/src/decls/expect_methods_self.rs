macro_rules! expect_methods_self {
    () => {
        macro_rules ! expect_methods_self { ($ ($ name : ident , $ ret_ty : ty , $ pat : pat , $ ret_val : expr ;) *) => { $ (# [track_caller] pub fn $ name (& self) -> $ ret_ty { let $ pat = self else { expect_failed (stringify ! ($ ident) , self) } ; $ ret_val }) * } }
    };
}

expect_methods_self!()