// Generated macro for error_type (macro)
macro_rules! Depcrate_utilerror_type {
() => {
// Module: crate::util
// Provides: {"error_type"}
// Dependencies: {}
macro_rules ! error_type { ($ name : ident) => { # [doc (hidden)] pub struct $ name { _inner : () , } impl :: std :: fmt :: Debug for $ name { fn fmt (& self , f : & mut :: std :: fmt :: Formatter) -> :: std :: fmt :: Result { f . debug_struct (stringify ! ($ name)) . finish () } } impl :: std :: fmt :: Display for $ name { fn fmt (& self , f : & mut :: std :: fmt :: Formatter) -> :: std :: fmt :: Result { f . write_str (stringify ! ($ name)) } } impl :: std :: error :: Error for $ name { } } ; }
};
}
