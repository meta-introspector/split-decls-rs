macro_rules! and {
    () => {
        # [doc = " Joins the arguments with `&&` operators."] macro_rules ! and { ($ ($ expr : expr) ,* $ (,) ?) => { $ ($ expr) &&* } ; }
    };
}

and!()