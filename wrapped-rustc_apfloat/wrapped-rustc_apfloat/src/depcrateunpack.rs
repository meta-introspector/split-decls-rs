// Generated macro for unpack (macro)
macro_rules! Depcrateunpack {
() => {
// Module: crate
// Provides: {"unpack"}
// Dependencies: {}
# [macro_export] macro_rules ! unpack { ($ status : ident |=, $ e : expr) => { match $ e { $ crate :: StatusAnd { status , value } => { $ status |= status ; value } } } ; ($ status : ident =, $ e : expr) => { match $ e { $ crate :: StatusAnd { status , value } => { $ status = status ; value } } } ; }
};
}
