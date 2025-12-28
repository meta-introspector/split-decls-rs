macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! __fallback_ensure {
    () => {
        deps!();
        # [doc (hidden)] # [macro_export] macro_rules ! __fallback_ensure { ($ cond : expr $ (,) ?) => { if $ crate :: __private :: not ($ cond) { return $ crate :: __private :: Err ($ crate :: Error :: msg ($ crate :: __private :: concat ! ("Condition failed: `" , $ crate :: __private :: stringify ! ($ cond) , "`"))) ; } } ; ($ cond : expr , $ msg : literal $ (,) ?) => { if $ crate :: __private :: not ($ cond) { return $ crate :: __private :: Err ($ crate :: __anyhow ! ($ msg)) ; } } ; ($ cond : expr , $ err : expr $ (,) ?) => { if $ crate :: __private :: not ($ cond) { return $ crate :: __private :: Err ($ crate :: __anyhow ! ($ err)) ; } } ; ($ cond : expr , $ fmt : expr , $ ($ arg : tt) *) => { if $ crate :: __private :: not ($ cond) { return $ crate :: __private :: Err ($ crate :: __anyhow ! ($ fmt , $ ($ arg) *)) ; } } ; }
    };
}

__fallback_ensure!();