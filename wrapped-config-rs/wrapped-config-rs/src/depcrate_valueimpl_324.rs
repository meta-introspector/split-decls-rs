// Generated macro for impl_324 (impl)
macro_rules! Depcrate_valueimpl_324 {
() => {
// Module: crate::value
// Provides: {"impl_324"}
// Dependencies: {}
impl Display for ValueKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use std :: fmt :: Write ; match * self { Self :: String (ref value) => write ! (f , "{value}") , Self :: Boolean (value) => write ! (f , "{value}") , Self :: I64 (value) => write ! (f , "{value}") , Self :: I128 (value) => write ! (f , "{value}") , Self :: U64 (value) => write ! (f , "{value}") , Self :: U128 (value) => write ! (f , "{value}") , Self :: Float (value) => write ! (f , "{value}") , Self :: Nil => write ! (f , "nil") , Self :: Table (ref table) => { let mut s = String :: new () ; for (k , v) in table . iter () { write ! (s , "{k} => {v}, ") ? ; } write ! (f , "{{ {s} }}") } Self :: Array (ref array) => { let mut s = String :: new () ; for e in array . iter () { write ! (s , "{e}, ") ? ; } write ! (f , "{s:?}") } } } }
};
}
