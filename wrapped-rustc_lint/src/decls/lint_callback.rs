macro_rules! lint_callback {
    () => {
        macro_rules ! lint_callback { ($ cx : expr , $ f : ident , $ ($ args : expr) ,*) => ({ $ cx . pass .$ f (&$ cx . context , $ ($ args) ,*) ; }) }
    };
}

lint_callback!()