macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! macro_106 {
    () => {
        deps!();
        # [cfg (doc)] __ensure ! [# [macro_export] macro_rules ! ensure { ($ cond : expr $ (,) ?) => { if !$ cond { return $ crate :: __private :: Err ($ crate :: Error :: msg ($ crate :: __private :: concat ! ("Condition failed: `" , $ crate :: __private :: stringify ! ($ cond) , "`"))) ; } } ; ($ cond : expr , $ msg : literal $ (,) ?) => { if !$ cond { return $ crate :: __private :: Err ($ crate :: __anyhow ! ($ msg)) ; } } ; ($ cond : expr , $ err : expr $ (,) ?) => { if !$ cond { return $ crate :: __private :: Err ($ crate :: __anyhow ! ($ err)) ; } } ; ($ cond : expr , $ fmt : expr , $ ($ arg : tt) *) => { if !$ cond { return $ crate :: __private :: Err ($ crate :: __anyhow ! ($ fmt , $ ($ arg) *)) ; } } ; }] ;
    };
}

macro_106!()