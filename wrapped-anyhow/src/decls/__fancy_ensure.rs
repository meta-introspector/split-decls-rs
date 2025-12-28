macro_rules! deps {
    () => {
        NotBothDebug!();
        BothDebug!();
    };
}

macro_rules! __fancy_ensure {
    () => {
        deps!();
        # [doc (hidden)] # [macro_export] macro_rules ! __fancy_ensure { ($ lhs : expr , $ op : tt , $ rhs : expr) => { match (&$ lhs , &$ rhs) { (lhs , rhs) => { if ! (lhs $ op rhs) { # [allow (unused_imports)] use $ crate :: __private :: { BothDebug , NotBothDebug } ; return Err ((lhs , rhs) . __dispatch_ensure ($ crate :: __private :: concat ! ("Condition failed: `" , $ crate :: __private :: stringify ! ($ lhs) , " " , $ crate :: __private :: stringify ! ($ op) , " " , $ crate :: __private :: stringify ! ($ rhs) , "`" ,) ,)) ; } } } } ; }
    };
}

__fancy_ensure!();