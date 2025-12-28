macro_rules! bail {
    () => {
        macro_rules ! bail { ($ item : expr , $ ($ msg : tt) ,*) => { return Err (syn :: Error :: new ($ item . span () , std :: fmt :: format (format_args ! ($ ($ msg) ,*)))) ; } ; }
    };
}

bail!()