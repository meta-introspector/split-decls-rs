macro_rules! err_validation_failure {
    () => {
        macro_rules ! err_validation_failure { ($ where : expr , $ kind : expr) => { { let where_ = &$ where ; let path = if ! where_ . is_empty () { let mut path = String :: new () ; write_path (& mut path , where_) ; Some (path) } else { None } ; err_ub ! (ValidationError (ValidationErrorInfo { path , kind : $ kind })) } } ; }
    };
}

err_validation_failure!();