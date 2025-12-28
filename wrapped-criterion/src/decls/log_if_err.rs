macro_rules! log_if_err {
    () => {
        # [doc = " Logs an error, ignores an `Ok` value."] macro_rules ! log_if_err { ($ x : expr) => { let closure = || { try_else_return ! ($ x) ; } ; closure () ; } ; }
    };
}

log_if_err!()