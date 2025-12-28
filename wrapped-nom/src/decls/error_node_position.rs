macro_rules! error_node_position {
    () => {
        # [doc = " Creates a parse error from a `nom::ErrorKind`,"] # [doc = " the position in the input and the next error in"] # [doc = " the parsing tree"] # [allow (unused_variables)] # [macro_export (local_inner_macros)] macro_rules ! error_node_position (($ input : expr , $ code : expr , $ next : expr $ (,) ?) => ({ $ crate :: error :: append_error ($ input , $ code , $ next) }) ;) ;
    };
}

error_node_position!()