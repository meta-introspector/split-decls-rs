macro_rules! safe_println {
    () => {
        macro_rules ! safe_println { ($ ($ arg : tt) *) => { safe_print ! ("{}\n" , std :: format_args ! ($ ($ arg) *)) } ; }
    };
}

safe_println!();