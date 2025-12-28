macro_rules! methods_target_is_kind {
    () => {
        macro_rules ! methods_target_is_kind { ($ ($ name : ident => $ kind : expr) ,*) => { $ (# [doc = " Return true if this target is of kind `$kind`."] pub fn $ name (& self) -> bool { self . is_kind ($ kind) }) * } }
    };
}

methods_target_is_kind!();