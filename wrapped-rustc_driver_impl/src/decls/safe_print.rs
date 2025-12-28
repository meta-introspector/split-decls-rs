macro_rules! safe_print {
    () => {
        macro_rules ! safe_print { ($ ($ arg : tt) *) => { { $ crate :: print :: print (std :: format_args ! ($ ($ arg) *)) ; } } ; }
    };
}

safe_print!();