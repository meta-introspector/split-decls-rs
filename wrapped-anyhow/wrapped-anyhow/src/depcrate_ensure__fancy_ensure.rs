// Generated macro for __fancy_ensure (macro)
macro_rules! Depcrate_ensure__fancy_ensure {
() => {
// Module: crate::ensure
// Provides: {"__fancy_ensure"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __fancy_ensure { ($ lhs : expr , $ op : tt , $ rhs : expr) => { match (&$ lhs , &$ rhs) { (lhs , rhs) => { if ! (lhs $ op rhs) { # [allow (unused_imports)] use $ crate :: __private :: { BothDebug , NotBothDebug } ; return Err ((lhs , rhs) . __dispatch_ensure ($ crate :: __private :: concat ! ("Condition failed: `" , $ crate :: __private :: stringify ! ($ lhs) , " " , $ crate :: __private :: stringify ! ($ op) , " " , $ crate :: __private :: stringify ! ($ rhs) , "`" ,) ,)) ; } } } } ; }
};
}
