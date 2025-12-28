macro_rules! forward_writes {
    () => {
        # [cfg (feature = "default-hasher")] macro_rules ! forward_writes { ($ ($ write : ident ($ ty : ty) ,) *) => { $ (# [inline (always)] fn $ write (& mut self , arg : $ ty) { self . inner .$ write (arg) ; }) * } }
    };
}

forward_writes!()