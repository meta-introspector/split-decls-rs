macro_rules! try_else_return {
    () => {
        # [doc = " Matches a result, returning the `Ok` value in case of success,"] # [doc = " exits the calling function otherwise."] # [doc = " A closure which returns the return value for the function can"] # [doc = " be passed as second parameter."] macro_rules ! try_else_return { ($ x : expr) => { try_else_return ! ($ x , || { }) } ; ($ x : expr , $ el : expr) => { match $ x { Ok (x) => x , Err (e) => { crate :: error :: log_error (& e) ; let closure = $ el ; return closure () ; } } } ; }
    };
}

try_else_return!();