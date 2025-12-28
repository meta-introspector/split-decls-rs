macro_rules! or {
    () => {
        # [doc = " Joins the arguments with `||` operators."] # [cfg (feature = "terminfo")] macro_rules ! or { ($ ($ expr : expr) ,* $ (,) ?) => { $ ($ expr) ||* } ; }
    };
}

or!();