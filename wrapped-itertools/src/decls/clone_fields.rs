macro_rules! clone_fields {
    () => {
        macro_rules ! clone_fields { ($ ($ field : ident) ,*) => { # [inline] fn clone (& self) -> Self { Self { $ ($ field : self .$ field . clone () ,) * } } } }
    };
}

clone_fields!()