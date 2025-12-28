macro_rules! error_position {
    () => {
        # [doc = " Creates a parse error from a `nom::ErrorKind`"] # [doc = " and the position in the input"] # [allow (unused_variables)] # [macro_export (local_inner_macros)] macro_rules ! error_position (($ input : expr , $ code : expr $ (,) ?) => ({ $ crate :: error :: make_error ($ input , $ code) }) ;) ;
    };
}

error_position!();