macro_rules! unexpected_token {
    () => {
        macro_rules ! unexpected_token { ($ item : expr , $ msg : expr) => { if let Some (i) = $ item { bail ! (i , "unexpected {}" , $ msg) ; } } ; }
    };
}

unexpected_token!();