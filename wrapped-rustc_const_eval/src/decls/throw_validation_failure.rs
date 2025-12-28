macro_rules! throw_validation_failure {
    () => {
        macro_rules ! throw_validation_failure { ($ where : expr , $ kind : expr) => { do yeet err_validation_failure ! ($ where , $ kind) } ; }
    };
}

throw_validation_failure!()