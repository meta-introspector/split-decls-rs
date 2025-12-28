macro_rules! try_right {
    () => {
        # [doc = " Dual to [`try_left!`], see its documentation for more information."] # [macro_export] macro_rules ! try_right { ($ expr : expr) => { match $ expr { $ crate :: Left (err) => return $ crate :: Left (:: core :: convert :: From :: from (err)) , $ crate :: Right (val) => val , } } ; }
    };
}

try_right!();